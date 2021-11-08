# learn-rust
rust 学习

### 表达式

rust 的表达式很特殊，比如 `1 + 1` 是一个表达。`let y = 6 ` 中的 6 是一个表达式，他计算出来的值是 6 赋值给 `y`。

包括 `()` 以及 `{}` 都是一种表达式

```rust
let y = {
        let x = 3;
        x + 1
    };
```

其中 

```rust
{
        let x = 3;
        x + 1
    };
```

是一个表达式，最终会计算出来 4 赋值给 `y`，其中 `x+1` 并没有分号结尾，这是区分表达式和代码块，如果加上分号则会变成代码块

### 具有返回值的函数

在 rust 中一个函数需要返回值的话，是在函数括号后加上类型，则函数就会返回最后一行表达式计算出来的值，如下例

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
// x -> 5
```
其中要注意最后一行表达式不能带分号

### 理解 Ownership/Reference/Borrowing

### 理解 mod

一个 rust 项目就是一个 crate, 一个 crate 可以有多个 mod， 相当于一棵树，而 bin 和 src 是两个不同的 crate ，包括 examples 或者 tests 下面要用到 src 里面的东西都需要显示的定义 mod 把对应的接口导出到 lib.rs 然后才能在这些目录使用。
