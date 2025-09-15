//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
fn main() {
    // 这样当 build.rs 改变时会重新运行（便于调试）
    println!("cargo:rerun-if-changed=build.rs");

    // 为编译器定义一个 cfg：feature = "pass"
    // 这样 #[cfg(feature = "pass")] 分支会被开启
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

