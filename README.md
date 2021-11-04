## study-rust

> [Rust程序设计语言](https://kaisery.github.io/trpl-zh-cn/title-page.html)

### Mac 安装 rust

1. 通过`brew`安装

```bash
$ brew install rust
```

2. 检验安装是否成功

```bash
$ rustc --version
// rustc 1.50.0
$ cargo --version
// 1.50.0
```

3. 验证

```bash
$ echo 'fn main() { println!("Hello, world!"); }' > test.rs && rustc test.rs && ./test
// Hello, world!  
$ rm -rf test test.rs
```

---

### 更换国内源

```bash
$ vim ~/.cargo/config
```

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'ustc'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"
```

---

### 常用命令

- 编译为可执行程序

```bash
$ rustc src/main.rs
```

- 创建新的 rust 项目

```bash
$ cargo new 项目名
```

- 启动项目

```bash
$ cargo run
```

## 变量与语法

- `rust` 中的函数和变量名使用`snake case`规范，要求所有字母小写并使用下划线分隔单词

### 声明变量

- `let` 声明变量
    - 声明后不可更改；
    - 可变变量需要在名称之前增加`mut`关键字；
    - 在同一作用域下，可重复使用`let`关键字声明多个同名变量，会覆盖上一个声明；
    - 父子级作用域下不会覆盖
    - `let mut num = 1;`

- `const` 声明常量
    - 声明后不可更改；
    - 需要显式指定常量类型；
    - 按照规范，常量应全部使用大写字母；
    - `const NUMBER: u32 = 1;`

---

### 数据类型

- 标量（数字类型）
    - 整数
        - 整形分为「有符号」和「无符号」，已`i`和`u`作为区分
        - 类型：`u8` `u16` `u32` `u64` `u128` `arch`
    - 浮点数
        - 类型：`f32` `f64`

- 布尔值
    - 类型：`bool`
    - 值：`true` `false`

- 元祖
    - 固定长度的类数组类型，每个位置的类型可以不同
    - `let tuple: (u32, bool) = (1, true);`

- 数组
    - 固定长度，声明后不可变更，每个位置的类型必须相同
    - `let array: [u32, 2] = [0, 1];`

- 函数
    - 函数已`fn`关键字声明

--- 

### 控制流

- `if` 条件判断
    - `if`关键字后面直接跟着判断语句，不需要括号包裹
    - `if`后面的判断语句值必须是`bool`类型
    - `if`可以用于变量声明（类似三元运算符）：`let num = if condition { 1 } else { 2 }`
- `loop` 循环
    - `loop`循环会一直重复执行代码块里面的内容直到代码中明确要求停止
    - 循环停止的关键字为：`break` `continue`
    - `loop`可以通过`break`关键字用于变量赋值：`let num = loop { break 1 };`
- `while` 循环
    - `while`与其他语言一致，当条件为真时重复执行
- `for` 循环
  ```rust
  let nums = [1, 2, 3, 4, 5];
  for item in nums.iter() {
      println!("item is {}", item)
  }
  ```

---

