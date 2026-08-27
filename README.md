***
*此文件由AI生成，请仔细甄别*
***

# rustbf

一个用 Rust 编写的简单 Brainfuck 解释器与库（CLI + lib），适合学习、嵌入或轻量测试 Brainfuck 程序。

## 用途（What this is）

rustbf 用尽量精简的代码实现了一个可执行的 Brainfuck 解释器，并将解释器核心放在 `src/lib.rs` 以便复用。你可以：

- 直接使用仓库提供的二进制（CLI）从文件运行 Brainfuck 程序。
- 在其它 Rust 程序中以库的形式调用 `run_bf`。

## 技术栈（Stack）

- 语言：Rust
- Edition：2024
- 主要实现：仅使用标准库（无额外依赖）

## 仓库结构（How it's organized）

```
Cargo.toml      # cargo package 配置
Cargo.lock
src/
  main.rs       # CLI：从文件读取 BF 源码并调用 run_bf
  lib.rs        # 解释器实现：pub fn run_bf(src: &str, input: Option<Vec<char>>) -> Result<Vec<char>, String>
```

How it fits together:

- `main.rs` 作为命令行入口：解析命令行参数（Brainfuck 源文件路径，可选 `--with-input`），读取文件内容并调用 `run_bf`，然后把返回的字符输出到 stdout。
- `lib.rs` 提供了解释器实现并导出 `run_bf`，便于在其他程序中直接复用。

## 特性与行为摘要

- 内存模型：使用 `Vec<u8>`，初始包含 1 个字节（值 0）。数据指针从 0 开始，向右移动时会按需扩容；向左移出边界会返回 Err（错误信息为 "the index is negative."）。
- 算术：对单元使用 `wrapping_add` / `wrapping_sub`，按 `u8` 溢出回绕（wrap-around）。
- I/O：`,` 指令优先从 `input` 参数取值；当 `input` 为 `None` 时从标准输入读取字节（阻塞），`.` 指令把当前单元按为 `char` 推入输出向量；`run_bf` 在返回前会在输出末尾追加一个换行符 `'\n'`。
- 循环 `[]`：通过查找匹配括号实现跳转。遇到未匹配的 `[` 或 `]` 会返回明确的 `Err`（"Unexpected token ..."）。

## 快速开始（How to run it）

在仓库根目录编译并运行 CLI：

```bash
# 直接用 cargo 运行（传入一个 .bf 源文件路径）
cargo run -- path/to/program.bf

# 或者构建 release 二进制后运行
cargo build --release
./target/release/rustbf path/to/program.bf
```

示例：一个常用的 Brainfuck 程序（打印 "Hello, World!" 风格的字符串，或可改成打印单个字符）：

```
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

将上面的程序保存为 `hello.bf`，然后运行：

```bash
cargo run -- hello.bf
```

如果程序需要输入（使用 `,`），可以在运行时从键盘输入，或者用 shell 重定向/管道提供输入：

```bash
# 将文件 input.bin 的内容作为程序 stdin
cargo run -- hello_with_input.bf < input.bin
```

也可以使用 `--with-input` 直接以字符列表形式提供输入（此时无需 stdin）：

```bash
# 提供 '1','2','3' 三个字符作为程序输入
cargo run -- hello_with_input.bf --with-input '1','2','3'
```

查看命令行用法：

```bash
cargo run -- -h
# 或
cargo run -- --help
```

## 作为库复用（二次使用 lib.rs）

如果你想在其他 Rust 项目中直接调用解释器：

方法 A - 本地路径依赖（本地开发）

在你的项目的 `Cargo.toml` 中添加：

```toml
[dependencies]
rustbf = { path = "../rustbf" }
```

在代码中使用：

```rust
use rustbf::run_bf;

fn main() {
    let program = ">+++."; // 一个简单的 BF 程序
    match run_bf(program, None) {
        Ok(output_chars) => {
            let s: String = output_chars.into_iter().collect();
            print!("{}", s);
        }
        Err(e) => eprintln!("运行时错误: {}", e),
    }
}
```

方法 B - 作为 Git 依赖

```toml
[dependencies]
rustbf = { git = "https://github.com/xiaoxiaoyang-114514/rustbf" }
```



## lib.rs API 说明 (重要)
 ```rust
 pub fn run_bf(src: &str, input: Option<Vec<char>>, stream_output: bool) -> Result<Option<Vec<char>>, String>
 ```

- 参数：
  - `src` - 包含 Brainfuck 源代码的字符串切片（解释器会忽略非 BF 指令字符）。
  - `input` - 可选输入，`Some(Vec<char>)` 时 `,` 指令从其中依次取字符；`None` 时从标准输入读取。
  - `stream_output` - 是否直接输出。`true` 时 `.` 指令直接打印到 stdout；`false` 时收集后返回。
- 返回值：
  - `Ok(Some(output))` - `stream_output` 为 false 时，运行结束时产生的输出字符序列（函数会在末尾追加一个换行符 `\n`）。
  - `Ok(None)` - `stream_output` 为 true 时，直接输出至 stdout，返回 None。
  - `Err(String)` - 在出现错误时返回，并附带错误信息

示例：把输出转换为字符串

```rust
let out = run_bf(">+.+.", None, false).unwrap().unwrap();
let s: String = out.into_iter().collect();
println!("输出: {}", s);
```

**注：当 `input` 为 `None` 时，`run_bf` 遇到 `,` 会读取控制台输入。若不想读取控制台，请传入 `Some(input)`。**

## 贡献与许可

欢迎提交 issue / PR。

---


