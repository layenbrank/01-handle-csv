use core::fmt;
// 引入标准库中的 Path 模块，用于文件路径的操作
use std::{path::Path, str::FromStr};

// 引入 clap 库中的 Parser 特性，用于命令行解析
use clap::Parser;

// 定义主命令选项 Opts，使用 clap 的 Parser 特性进行命令行解析
#[derive(Debug, Parser)]
// 这里定义了命令的名称、版本、作者、简介和长描述
#[command(name = "clitools", version, author, about, long_about = None)]
pub struct Opts {
    // 子命令，使用 command(subcommand) 属性
    #[command(subcommand)]
    pub cmd: Subcommand,
}

// 定义子命令枚举 Subcommand，其中包含一个 csv 子命令
#[derive(Debug, Parser)]
pub enum Subcommand {
    // csv 子命令，包含 CsvOpts 参数
    #[command(name = "csv", about = "将csv文件转换为其他格式")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
}

// 定义 CsvOpts 结构体，包含 CSV 处理所需的参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // 输入文件参数，使用 short 和 long 选项，并自定义验证函数 verify_input
    #[arg(short, long,value_parser = verify_input)]
    pub input: String,

    // 输出文件参数，默认值为 "output.json"
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long,value_parser=parser_format, default_value = "json")]
    pub format: Format,

    // 是否包含头信息，默认为 true
    #[arg(long, default_value_t = true)]
    pub header: bool,

    // 分隔符，默认为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

// 验证输入文件是否存在的函数，返回 Result 类型
pub fn verify_input(filename: &str) -> Result<String, &'static str> {
    // 检查文件是否存在
    if Path::new(filename).exists() {
        // 如果存在，返回 Ok 包含文件名的字符串
        Ok(filename.into())
    } else {
        // 如果不存在，返回 Err 包含错误信息的字符串
        Err("请检查该文件是否存在!")
    }
}

pub fn parser_format(format: &str) -> Result<Format, anyhow::Error> {
    format.parse()
}

impl From<Format> for &'static str {
    fn from(format: Format) -> Self {
        match format {
            Format::Json => "json",
            Format::Yaml => "yaml",
        }
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            format => Err(anyhow::anyhow!("不支持的格式 {}", format)),
        }
    }
}

// 命令行解析：使用 clap 库的 Parser 特性定义了命令行接口的结构。Opts 结构体包含一个 cmd 字段，它是 Subcommand 枚举的实例，代表了命令行的不同子命令。
// 子命令：Subcommand 枚举定义了一个 csv 子命令，它需要一个 CsvOpts 参数，用于指定 CSV 文件转换的选项。
// 参数验证：CsvOpts 结构体中的 input 字段使用了 value_parser 属性，指定了 verify_input 函数作为验证器，确保提供的输入文件确实存在。
// 参数默认值：CsvOpts 结构体的 output 和 delimiter 字段提供了默认值，分别为 "output.json" 和 ,。
// 通过这种方式，用户可以通过命令行调用 clitools 工具，并使用 csv 子命令来指定 CSV 文件的输入和输出以及其他选项。如果输入文件不存在，verify_input 函数将阻止程序继续执行并提示用户。

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
