use std::time::SystemTime;

pub fn randf32(seed: u32) -> f32 {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    ((seed ^ (t as u32) ^ 3631984913) % 100000) as f32 / 100000.0
}
