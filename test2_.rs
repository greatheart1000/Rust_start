下面我们一步步来，用 VS Code + rust-analyzer 插件，创建一个带测试的 `lib.rs` 并生成测试模板。

---

## 1. 用 Cargo 新建一个 Library 项目

打开终端，执行：  
```bash
cargo new my_crate --lib
cd my_crate
```

这时目录结构如下：  
```
my_crate/
├── Cargo.toml
└── src/
    └── lib.rs   ← 默认只有一个空的 lib.rs
```

---

## 2. 在 VS Code 中打开项目，并安装 rust-analyzer

1. 打开 VS Code，按 Ctrl+P 输入项目路径并回车。  
2. 扩展商店安装 “rust-analyzer” 插件（如果已装可跳过）。  
3. 确保状态栏有 “Rust: ready” 提示。

---

## 3. 在 `src/lib.rs` 写一个示例函数

编辑 `src/lib.rs`，加入下面这段代码：

```rust
// src/lib.rs

/// 给入参加 2
pub fn add_two(a: i32) -> i32 {
    a + 2
}
```

保存后，rust-analyzer 会对这个函数进行语法和类型检查。

---

## 4. 生成测试模块（手动或用 Code Action）

### 4.1 手动方式

在 `lib.rs` 末尾补充：

```rust
// 只在执行 `cargo test` 时编译和执行
#[cfg(test)]
mod tests {
    use super::*;   // 把外层所有 pub 项目都引入进来

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
}
```

### 4.2 用 VS Code 的 “Generate Tests” Code Action

1. 把光标放到函数名 `add_two` 上。  
2. 按 `Ctrl+.`（Windows/Linux）或 `⌘+.`（macOS）。  
3. 选择 “Generate tests for function `add_two`”。  
4. VS Code 会自动在文件底部插入一个 `#[cfg(test)] mod tests { … }` 的模板，并为你生成 `#[test] fn` 框架。  

---

## 5. 运行测试

在集成终端运行：

```bash
cargo test
```

输出示例：  
```
   Compiling my_crate v0.1.0 (/…/my_crate)
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/lib.rs (target/debug/deps/my_crate-…)
running 2 tests
test tests::add_two_and_two    ... ok
test tests::add_three_and_two  ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 6. 小贴士

- VS Code “Test Explorer” 插件配合 rust-analyzer，可以在编辑器左侧直接点击 ▶️ 运行单个测试。  
- 测试里除了 `assert_eq!`，还可以用 `assert!`, `assert_ne!`，以及 `#[should_panic]`、异步测试 (`#[tokio::test]`) 等。  
- 更多测试技巧可以参考官方文档：  
  https://doc.rust-lang.org/book/ch11-00-testing.html  

---

这样，你就完成了从零搭建、编写到执行 Rust 单元测试的全流程。
