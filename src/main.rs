fn main() {
    println!("=== Rust Result 错误处理演示 ===\n");

    // 1. Result 类型：可恢复的错误处理
    let result: Result<i32, ParseIntError> = "42".parse();
    match result {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    let bad_result: Result<i32, ParseIntError> = "not a number".parse();

    // 2. unwrap 和 expect
    let ok_value = result.unwrap(); // 成功时获取值
    println!("unwrap: {}", ok_value);

    // let panic_value = bad_result.unwrap(); // 会 panic！

    let expect_value = result.expect("解析应该成功"); // 自定义错误信息
    println!("expect: {}", expect_value);

    // 3. unwrap_or 提供默认值
    let value = bad_result.unwrap_or(0);
    println!("unwrap_or: {}", value);

    // 4. ? 操作符（传播错误）
    let result2 = read_and_parse("42");
    println!("read_and_parse: {:?}", result2);

    let result3 = read_and_parse("bad");
    println!("read_and_parse 错误: {:?}", result3);

    // 5. map 处理成功情况
    let mapped = result.map(|n| n * 2);
    println!("map: {:?}", mapped);

    // 6. and_then 链式处理
    let chained = result.and_then(|n| Ok(n + 10));
    println!("and_then: {:?}", chained);

    // 7. 自定义错误类型
    let custom_result = divide(10, 2);
    match custom_result {
        Ok(v) => println!("10 / 2 = {}", v),
        Err(e) => println!("错误: {}", e),
    }

    let custom_err = divide(10, 0);
    match custom_err {
        Ok(v) => println!("10 / 0 = {}", v),
        Err(e) => println!("预期错误: {}", e),
    }

    println!("\n=== 总结 ===");
    println!("Result<T, E> 用于可恢复的错误");
    println!("Ok(T) 表示成功，Err(E) 表示错误");
    println!("? 操作符简化错误传播");
    println!("丰富的组合器支持函数式风格");
}

use std::num::ParseIntError;

// 使用 ? 操作符的函数
fn read_and_parse(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// 自定义错误类型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeNumber,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else if a < 0 || b < 0 {
        Err(MathError::NegativeNumber)
    } else {
        Ok(a / b)
    }
}
