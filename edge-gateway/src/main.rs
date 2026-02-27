use anyhow::{Context, Result};
use rand::Rng;
use std::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let module = std::env::var("SPACETIME_MODULE").unwrap_or_else(|_| "smart-factory".to_string());
    let db_uri = std::env::var("SPACETIME_URI").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    let machine_id: u32 = std::env::var("MACHINE_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    println!("edge-gateway started: module={module}, uri={db_uri}, machine_id={machine_id}");
    println!("using `spacetime call` as transport (MVP)");

    loop {
        let mut rng = rand::thread_rng();
        let temp: f32 = rng.gen_range(55.0..95.0);
        let vib: f32 = rng.gen_range(10.0..80.0);
        let ts = now_ms();

        call_sensor_update(&db_uri, &module, machine_id, "temperature", temp, ts)?;
        call_sensor_update(&db_uri, &module, machine_id, "vibration", vib, ts)?;

        println!("sent temp={temp:.2}, vibration={vib:.2}, ts={ts}");
        sleep(Duration::from_secs(1)).await;
    }
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
