# up_finder

[![Crates.io](https://img.shields.io/crates/v/up_finder)](https://crates.io/crates/up_finder)
[![Documentation](https://docs.rs/up_finder/badge.svg)](https://docs.rs/up_finder)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

向上查找文件或目录的轻量级 Rust 工具库。

[English](README.md) | [中文](README.zh.md)

## 功能特点

- 在当前工作目录及所有父目录中查找文件或目录
- 支持查找单个文件或多个文件
- 使用 `rustc-hash` 提供高性能 HashMap 实现
- 简洁的构建器 API，使用 `typed-builder` 实现
- 无外部系统依赖，纯 Rust 实现

## 安装

将以下依赖添加到你的 `Cargo.toml` 文件中:

```toml
[dependencies]
up_finder = "0.0.1"
```

## 使用示例

### 查找单个文件

```rust
use up_finder::{UpFinder, FindUpKind};

// 创建一个 UpFinder 实例，默认查找文件
let find_up = UpFinder::builder()
    .cwd(".")  // 从当前目录开始
    .kind(FindUpKind::File)  // 可选，默认就是查找文件
    .build();

// 向上查找 package.json 文件
let paths = find_up.find_up("package.json");

// 打印找到的所有路径
println!("{:#?}", paths);
```

### 同时查找多个文件

```rust
use up_finder::{UpFinder, FindUpKind};

// 创建 UpFinder 实例
let find_up = UpFinder::builder()
    .cwd("./src")  // 从 src 目录开始
    .build();

// 同时查找多个文件
let paths = find_up.find_up_multi(&["package.json", ".gitignore", "Cargo.toml"]);

// 结果是一个 HashMap，键为文件名，值为找到的路径列表
for (file_name, file_paths) in paths {
    println!("找到 {} 个 {} 文件:", file_paths.len(), file_name);
    for path in file_paths {
        println!("  - {}", path.display());
    }
}
```

### 查找目录

```rust
use up_finder::{UpFinder, FindUpKind};

// 创建一个查找目录的 UpFinder 实例
let find_up = UpFinder::builder()
    .cwd(".")
    .kind(FindUpKind::Dir)  // 设置为查找目录
    .build();

// 向上查找名为 "node_modules" 的目录
let paths = find_up.find_up("node_modules");

println!("{:#?}", paths);
```

## API 文档

详细的 API 文档请访问 [docs.rs/up_finder](https://docs.rs/up_finder)。

## 贡献

欢迎提交问题和拉取请求！

## 许可证

本项目使用 [MIT 许可证](LICENSE) 进行许可。 