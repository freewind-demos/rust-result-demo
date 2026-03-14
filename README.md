# Rust Result Demo

## 简介

演示 Rust 的 Result 类型和错误处理。

## 基本原理

Result<T, E> 用于可恢复的错误：
- Ok(T) - 成功
- Err(E) - 错误

## 启动和使用

```bash
cargo run
```

## 教程

### 基本用法

```rust
let result: Result<i32, ParseIntError> = "42".parse();
match result {
    Ok(n) => println!("解析成功: {}", n),
    Err(e) => println!("错误: {}", e),
}
```

### ? 操作符

```rust
fn read_and_parse(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}
```

### unwrap 和 expect

```rust
let value = result.unwrap();     // panic if Err
let value = result.expect("msg"); // panic with msg
let value = result.unwrap_or(0);  // default value
```
