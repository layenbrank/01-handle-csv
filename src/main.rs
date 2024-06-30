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
        Subcommand::Csv(csv_opts) => {
            // 根据用户是否提供了输出文件名，确定最终的输出文件名。
            let output = if let Some(output) = &csv_opts.output {
                output.clone()
            } else {
                // 如果没有提供输出文件名，使用默认格式 "output.<format>"
                format!("output.{}", csv_opts.format)
            };
            // 调用 process_csv 函数，传递输入文件路径、输出文件路径和格式。
            // 注意：这里的 csv_opts.input 和 output 是所有权移动，而不是引用。
            process_csv(&csv_opts.input, output, csv_opts.format)?;
            // 如果 process_csv 函数抛出错误，会通过 ? 操作符被捕获并向上抛出。
        }
    }
    // 如果没有匹配到任何子命令，或者所有子命令都成功执行完毕，则返回 Ok(()).
    Ok(())
}

// 关键点包括：
// - 使用 clap::Parser 来解析命令行参数，生成 Opts 结构体实例。
// - Opts 结构体和 Subcommand 枚举来自 clitoos 库，用于定义命令行接口的结构和行为。
// - process_csv 函数用于处理 CSV 文件，接受输入文件路径、输出文件路径和输出格式作为参数。
// - 使用 anyhow::Result 来处理和传播错误，通过 ? 操作符将内部函数的错误向上抛出，确保错误处理的一致性。
