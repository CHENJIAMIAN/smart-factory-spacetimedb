use anyhow::{Context, Result};
use rand::Rng;
use serde::Serialize;
use std::{fs, path::PathBuf, process::Command};
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct LiveMachine {
    id: u32,
    name: String,
    status: String,
    oee: f32,
    temperature: f32,
    vibration: f32,
    last_updated: u64,
}

#[derive(Serialize)]
struct LiveAlert {
    level: String,
    message: String,
    timestamp: u64,
}

#[derive(Serialize)]
struct LivePayload {
    machine: LiveMachine,
    alerts: Vec<LiveAlert>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let module = std::env::var("SPACETIME_MODULE").unwrap_or_else(|_| "smart-factory".to_string());
    let db_uri = std::env::var("SPACETIME_URI").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let machine_id: u32 = std::env::var("MACHINE_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let call_enabled = std::env::var("SPACETIME_CALL_ENABLED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let live_path = std::env::var("LIVE_JSON_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../dashboard/public/live.json"));

    println!("edge-gateway started: module={module}, uri={db_uri}, machine_id={machine_id}");
    println!("spacetime reducer call enabled: {call_enabled}");
    println!("live json path: {}", live_path.display());

    if let Some(parent) = live_path.parent() {
        fs::create_dir_all(parent)?;
    }

    loop {
        let mut rng = rand::thread_rng();
        let temp: f32 = rng.gen_range(55.0..95.0);
        let vib: f32 = rng.gen_range(10.0..80.0);
        let ts = now_ms();

        let mut alerts = vec![];
        let status = if temp > 85.0 {
            alerts.push(LiveAlert {
                level: "critical".to_string(),
                message: format!("Machine {} overheat: {:.2}°C", machine_id, temp),
                timestamp: ts,
            });
            "fault"
        } else if vib > 60.0 {
            alerts.push(LiveAlert {
                level: "warning".to_string(),
                message: format!("Machine {} vibration high: {:.2}", machine_id, vib),
                timestamp: ts,
            });
            "warning"
        } else {
            "running"
        }
        .to_string();

        let oee = estimate_oee(temp, vib);

        if call_enabled {
            if let Err(err) = call_sensor_update(&db_uri, &module, machine_id, "temperature", temp, ts) {
                eprintln!("warn: reducer call temperature failed: {err}");
            }
            if let Err(err) = call_sensor_update(&db_uri, &module, machine_id, "vibration", vib, ts) {
                eprintln!("warn: reducer call vibration failed: {err}");
            }
        }

        let payload = LivePayload {
            machine: LiveMachine {
                id: machine_id,
                name: format!("CNC-{machine_id:02}"),
                status,
                oee,
                temperature: temp,
                vibration: vib,
                last_updated: ts,
            },
            alerts,
        };

        fs::write(&live_path, serde_json::to_string_pretty(&payload)?)?;

        println!("sent temp={temp:.2}, vibration={vib:.2}, ts={ts}, oee={oee:.1}");
        sleep(Duration::from_secs(1)).await;
    }
}

fn estimate_oee(temp: f32, vib: f32) -> f32 {
    let temp_penalty = ((temp - 60.0).max(0.0) * 0.9).min(45.0);
    let vib_penalty = (vib * 0.7).min(45.0);
    (100.0 - temp_penalty - vib_penalty).clamp(0.0, 100.0)
}

fn call_sensor_update(
    db_uri: &str,
    module: &str,
    machine_id: u32,
    sensor_type: &str,
    value: f32,
    ts: u64,
) -> Result<()> {
    let output = Command::new("spacetime")
        .args([
            "call",
            "--uri",
            db_uri,
            module,
            "sensor_update",
            &machine_id.to_string(),
            sensor_type,
            &format!("{value:.3}"),
            &ts.to_string(),
        ])
        .output()
        .with_context(|| "failed to execute `spacetime` cli, make sure it is installed and in PATH")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("spacetime call failed: {err}");
    }

    Ok(())
}

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
