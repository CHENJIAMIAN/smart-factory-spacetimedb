use spacetimedb::{reducer, table, ReducerContext, Table};

#[table(name = machine, public)]
pub struct Machine {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub status: String, // running / idle / fault
    pub oee: f32,
    pub last_updated: u64,
}

#[table(name = sensor_data, public)]
pub struct SensorData {
    #[auto_inc]
    #[primary_key]
    pub id: u64,
    pub machine_id: u32,
    pub sensor_type: String, // temperature / vibration / power
    pub value: f32,
    pub timestamp: u64,
}

#[table(name = alert, public)]
pub struct Alert {
    #[auto_inc]
    #[primary_key]
    pub id: u64,
    pub machine_id: u32,
    pub level: String, // warning / critical
    pub message: String,
    pub timestamp: u64,
}

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    if ctx.db.machine().id().find(&1).is_none() {
        ctx.db.machine().insert(Machine {
            id: 1,
            name: "CNC-01".to_string(),
            status: "idle".to_string(),
            oee: 0.0,
            last_updated: now_ms(),
        });
    }
}

#[reducer]
pub fn sensor_update(
    ctx: &ReducerContext,
    machine_id: u32,
    sensor_type: String,
    value: f32,
    timestamp: u64,
) {
    ctx.db.sensor_data().insert(SensorData {
        id: 0,
        machine_id,
        sensor_type: sensor_type.clone(),
        value,
        timestamp,
    });

    let mut status = "running".to_string();
    if sensor_type == "temperature" && value > 85.0 {
        status = "fault".to_string();
        ctx.db.alert().insert(Alert {
            id: 0,
            machine_id,
            level: "critical".to_string(),
            message: format!("Machine {} overheat: {:.2}°C", machine_id, value),
            timestamp,
        });
    } else if sensor_type == "vibration" && value > 60.0 {
        status = "warning".to_string();
        ctx.db.alert().insert(Alert {
            id: 0,
            machine_id,
            level: "warning".to_string(),
            message: format!("Machine {} vibration high: {:.2}", machine_id, value),
            timestamp,
        });
    }

    if let Some(mut m) = ctx.db.machine().id().find(&machine_id) {
        m.status = status;
        m.oee = estimate_oee(value, &sensor_type);
        m.last_updated = timestamp;
        ctx.db.machine().id().update(m);
    }
}

fn estimate_oee(value: f32, sensor_type: &str) -> f32 {
    match sensor_type {
        "temperature" => (100.0 - ((value - 60.0).max(0.0) * 0.8)).clamp(0.0, 100.0),
        "vibration" => (100.0 - (value * 1.2)).clamp(0.0, 100.0),
        _ => 90.0,
    }
}

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
