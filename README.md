# Rust_start
## RUST语言入门
### Rust学习资料 https://kaisery.github.io/trpl-zh-cn/ch01-02-hello-world.html
wsl 环境下 安装Rust curl -4 --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh <br>
配置环境变量 source $HOME/.cargo/env  <br>
验证安装：   <br>
rustc --version <br>
cargo --version <br>
cd /mnt/d/project/Rust_project/lession01  <br>
使用 cargo init 命令。这个命令会在当前目录初始化一个新的 Cargo 项目，并创建 Cargo.toml 文件和 src 目录 <br>
可以使用 cargo new 创建项目。 <br>
可以使用 cargo build 构建项目。 <br>
可以使用 cargo run 一步构建并运行项目。 <br>
可以使用 cargo check 在不生成二进制文件的情况下构建项目来检查错误。 <br>
