use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u128 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}
