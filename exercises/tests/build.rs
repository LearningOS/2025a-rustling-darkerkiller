//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 在 build.rs 修改后重新运行 build 脚本（方便调试）
    println!("cargo:rerun-if-changed=build.rs");

    // 当前 UNIX 时间（秒）
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();

    // 将时间注入到被编译的 crate，使 std::env::var("TEST_FOO") 可在运行时读取到该值
    println!("cargo:rustc-env=TEST_FOO={}", ts);

    // 为 tests8.rs 中的 #[cfg(feature = "pass")] 打开条件分支
    println!("cargo:rustc-cfg=feature=\"pass\"");
}


