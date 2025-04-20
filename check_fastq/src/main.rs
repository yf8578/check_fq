use std::path::PathBuf;
use clap::{Parser, Subcommand};
use check_fastq::{check_fastq_file, FastqError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 检查 FASTQ 文件格式
    Check {
        /// 输入的 FASTQ 文件路径
        #[arg(short, long)]
        input: PathBuf,
        
        /// 错误输出文件路径（可选）
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), FastqError> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Check { input, output } => {
            println!("正在检查 FASTQ 文件: {}", input.display());
            
            match check_fastq_file(input, output.as_ref()) {
                Ok((record_count, error_count)) => {
                    println!("检查完成！");
                    println!("处理的记录总数: {}", record_count);
                    
                    if error_count == 0 {
                        println!("未发现错误。");
                    } else {
                        println!("发现 {} 条错误记录。", error_count);
                        if let Some(out_path) = output {
                            println!("错误记录已写入: {}", out_path.display());
                        }
                    }
                    
                    Ok(())
                }
                Err(e) => {
                    eprintln!("处理文件时出错: {}", e);
                    Err(e)
                }
            }
        }
    }
}