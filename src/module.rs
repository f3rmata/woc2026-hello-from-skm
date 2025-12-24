//! Module helper functions

use kernel::prelude::*;

const __LOG_PREFIX: &[u8] = b"woc2026_hello_from_rkm\0";

/// Test function that prints a message
#[allow(dead_code)]
pub(crate) fn test() {
    pr_info!("Rust LKM Template (test)\n");
}
