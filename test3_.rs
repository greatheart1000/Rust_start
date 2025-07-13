在 Rust 里，你可以用 `cargo test <FILTER>` 来只运行名称中包含 `<FILTER>` 的测试用例。下面先给出一个 `lib.rs` 的示例，演示如何用 `assert_eq!`、`assert!`、`assert_ne!` 和 `#[should_panic]`，然后再给出按名称过滤运行的命令。

```rust
// src/lib.rs

/// 被测试的函数
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // assert_eq!: 检查两个值相等
    #[test]
    fn test_add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    // assert!: 检查布尔表达式为 true
    #[test]
    fn test_truthy() {
        assert!(1 + 1 == 2);
    }

    // assert_ne!: 检查两个值不相等
    #[test]
    fn test_not_equal() {
        assert_ne!(5, add_two(2)); // 5 ≠ 4
    }

    // #[should_panic]: 期望测试中会 panic
    #[test]
    #[should_panic]
    fn test_will_panic() {
        panic!("this test should panic");
    }

    // 带 expected 的 #[should_panic]
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        // 模拟运行时 panic，比如数组越界、除以 0 等
        let _ = [][0]; // 越界会 panic
    }
}
```

保存后，所有测试都可以用：

```bash
# 运行全部测试
cargo test
```

按名字过滤运行：

```bash
# 只运行名称中包含 "add_two" 的测试
cargo test add_two

# 只运行精确匹配的 test_add_two_and_two
cargo test test_add_two_and_two

# 运行名称包含 "panic" 的测试（这里会跑两个 should_panic 测试）
cargo test panic
```

在输出里，你会看到类似：

```text
running 2 tests
test tests::test_add_two_and_two ... ok
test tests::test_not_equal        ... ok

test result: ok. 2 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out
```

（被过滤掉的测试会被标记为 ignored。）

---

小贴士：

- 过滤器是对测试函数名称做**子串匹配**，不区分大小写。
- 如果想看更详细的输出可以加 `-- --nocapture`：  
  `cargo test test_add_two_and_two -- --nocapture`  
- `#[should_panic(expected = "…")]` 可以精确匹配 panic 信息。
