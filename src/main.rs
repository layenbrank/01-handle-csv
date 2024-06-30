// 导入 clap 库中的 Parser 特性，用于解析命令行参数。
use clap::Parser;
// 导入 clitoos 库中的 process_csv 函数和 Opts 结构体，以及 Subcommand 枚举。
use clitoos::{process_csv, Opts, Subcommand};

// 主函数，返回一个 Result 类型，表示可能成功或失败的结果。
fn main() -> anyhow::Result<()> {
    // 解析命令行参数，并将其转换为 Opts 结构体实例。
    let opts = Opts::parse();
    // 根据 Opts 结构体中的 cmd 字段匹配不同的子命令。
    match opts.cmd {
        // 如果子命令是 Csv，则调用 process_csv 函数处理 CSV 文件。
        // 注意：&opts.input 和 &opts.output 是引用类型，传递给 process_csv 函数。
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    // 返回 Ok(()) 表示程序成功执行完毕。
    Ok(())
}

// 这里的关键点包括：

// 使用 clap::Parser 来解析命令行参数。
// Opts 结构体和 Subcommand 枚举来自 clitoos 库，用于定义命令行接口。
// process_csv 函数用于处理 CSV 文件，其具体实现未在代码中给出，但可以假设它接受输入文件路径和输出文件路径作为参数。
// 使用 anyhow::Result 来处理错误，通过 ? 操作符将内部函数的错误向上抛出。
