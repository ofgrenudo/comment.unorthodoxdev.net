use log::{error, info, warn};
pub mod create;
pub mod read;
pub mod update;
pub mod delete;

pub fn placeholder() -> i32 {
    info!("[COMMENT] returning 1");
    return 1
}