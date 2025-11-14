//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置 TEST_FOO 环境变量为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 告诉 Cargo 如果 build.rs 文件本身没有变化，不需要重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs");

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}