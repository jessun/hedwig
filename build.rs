use std::process::Command;

fn main() {
    // 1. 执行 git 命令获取 short hash
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    let git_hash = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "unknown".to_string(), // 如果没有安装 git 或不是 git 仓库
    };

    // 2. 将这个值设置为编译时的环境变量
    // 在 Rust 代码中可以使用 env!("GIT_HASH") 读取它
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
