pub mod double_buffering;
pub mod ring;
pub mod three_adder;

use std::time::Duration;

#[inline]
async fn sleep<const SLEEP: bool>() {
    if SLEEP {
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}
