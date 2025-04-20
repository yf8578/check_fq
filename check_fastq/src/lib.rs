use thiserror::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// 表示 FASTQ 中的一条序列记录
#[derive(Debug)]
pub struct FastqRecord {
    pub header: String,    // 以 @ 开头的序列标识符
    pub sequence: String,  // 序列数据
    pub plus_line: String, // 以 + 开头的行
    pub quality: String,   // 质量值
}

/// FASTQ 解析和验证过程中可能发生的错误
#[derive(Error, Debug)]
pub enum FastqError {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("格式错误: {0}")]
    Format(String),
    
    #[error("标题行 (行 {0}) 没有以 @ 开头")]
    InvalidHeader(usize),
    
    #[error("加号行 (行 {0}) 没有以 + 开头")]
    InvalidPlusLine(usize),
    
    #[error("序列长度 ({seq_len}) 与质量值长度 ({qual_len}) 不匹配 (行 {line_num})")]
    LengthMismatch { seq_len: usize, qual_len: usize, line_num: usize },
}

/// 验证 FASTQ 记录是否格式正确
pub fn validate_record(record: &FastqRecord, line_num: usize) -> Result<(), FastqError> {
    // 检查标题行是否以 @ 开头
    if !record.header.starts_with('@') {
        return Err(FastqError::InvalidHeader(line_num));
    }
    
    // 检查加号行是否以 + 开头
    if !record.plus_line.starts_with('+') {
        return Err(FastqError::InvalidPlusLine(line_num + 2));
    }
    
    // 检查序列长度与质量值长度是否一致
    if record.sequence.len() != record.quality.len() {
        return Err(FastqError::LengthMismatch {
            seq_len: record.sequence.len(),
            qual_len: record.quality.len(),
            line_num: line_num + 3,
        });
    }
    
    Ok(())
}
/// 解析 FASTQ 文件并验证其格式
pub fn check_fastq_file<P: AsRef<Path>>(
    input_path: P,
    error_output_path: Option<P>,
) -> Result<(usize, usize), FastqError> {
    let file = File::open(input_path.as_ref())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let mut line_num = 0;
    let mut record_count = 0;
    let mut error_count = 0;
    
    // 创建错误输出文件（如果需要）
    let mut error_file = if let Some(path) = error_output_path {
        Some(File::create(path)?)
    } else {
        None
    };
    
    // 逐条读取和验证 FASTQ 记录
    while let Some(Ok(header)) = lines.next() {
        line_num += 1;
        
        // 尝试读取完整的记录（4行）
        let sequence = match lines.next() {
            Some(Ok(seq)) => seq,
            _ => return Err(FastqError::Format(format!("在行 {} 之后意外结束", line_num))),
        };
        
        let plus_line = match lines.next() {
            Some(Ok(plus)) => plus,
            _ => return Err(FastqError::Format(format!("在行 {} 之后意外结束", line_num + 1))),
        };
        
        let quality = match lines.next() {
            Some(Ok(qual)) => qual,
            _ => return Err(FastqError::Format(format!("在行 {} 之后意外结束", line_num + 2))),
        };
        
        // 创建和验证记录
        let record = FastqRecord {
            header,
            sequence,
            plus_line,
            quality,
        };
        
        record_count += 1;
        
        // 验证记录并处理错误
        if let Err(err) = validate_record(&record, line_num) {
            error_count += 1;
            
            // 如果提供了错误输出文件，则写入错误记录
            if let Some(ref mut file) = error_file {
                writeln!(file, "错误: {:?}", err)?;
                writeln!(file, "{}", record.header)?;
                writeln!(file, "{}", record.sequence)?;
                writeln!(file, "{}", record.plus_line)?;
                writeln!(file, "{}", record.quality)?;
                writeln!(file, "---")?;
            }
        }
        
        line_num += 3; // 我们已经处理了4行
    }
    
    Ok((record_count, error_count))
}