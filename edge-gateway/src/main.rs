use anyhow::Result;
use rand::Rng;
use tokio::time::{sleep, Duration};

// 这里是示意：按你本机 SDK 的实际 API 替换
#[tokio::main]
async fn main() -> Result<()> {
    println!("edge-gateway started");
    println!("TODO: connect to SpacetimeDB endpoint and call reducer: sensor_update");

    loop {
        let mut rng = rand::thread_rng();
        let temp: f32 = rng.gen_range(55.0..95.0);
        let vib: f32 = rng.gen_range(10.0..80.0);

        let ts = now_ms();

        // pseudo:
        // client.call_reducer("sensor_update", (1u32, "temperature", temp, ts)).await?;
        // client.call_reducer("sensor_update", (1u32, "vibration", vib, ts)).await?;

        println!("send temp={temp:.2}, vibration={vib:.2}, ts={ts}");
        sleep(Duration::from_secs(1)).await;
    }
}

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
