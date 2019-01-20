# rust_study 的学习笔记

中文指南 <https://kaisery.gitbooks.io/rust-book-chinese/content/>

安装地址:
<https://www.rust-lang.org/tools/install>

## 环境安装 (win10)

<https://www.rust-lang.org/tools/install>

## 环境安装 (win10 WSL)

```bash shell
#安装
sudo curl https://sh.rustup.rs -sSf | sh
#卸载
rustup self uninstall
sudo apt remove rustc
```

## 使用方法

进入 Cargo.toml 所在目录

```bash shell
    #构建程序
    cargo build
    #执行程序
    cargo run
    #发布构建
    cargo build --release
    #创建项目
    cargo new hello_world --bin
```

## git 忽略

```git
#rust
target/
Cargo.lock
```
