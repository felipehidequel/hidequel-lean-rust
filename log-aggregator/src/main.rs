use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive (Debug, PartialEq)]
enum LogLevel{
    Info,
    Warning,
    Error,
} 

#[derive(Debug)]
struct LogEntry {
    level: LogLevel,
    timestamp: String,
    message: String,
}

#[derive(Debug)]
enum LogError{
    FormatoInvalido,
    LevelDesconhecido,
}

fn parse_log_line(line: &str) -> Result<LogEntry, LogError> {
    let parts : Vec<&str> = line.split(' ').collect();
    
    let data = parts.get(0).ok_or(LogError::FormatoInvalido)?;
    let hora = parts.get(1).ok_or(LogError::FormatoInvalido)?;
    let level_str = parts.get(2).ok_or(LogError::FormatoInvalido)?;

    let mensagem_parts = parts.get(3..).ok_or(LogError::FormatoInvalido)?;
    let message = mensagem_parts.join(" ");

    let timestamp = format!("{} {}", data, hora);
     
    let level = match *level_str{
        "[INFO]" => LogLevel::Info,
        "[WARN]" => LogLevel::Warning,
        "[ERROR]" => LogLevel::Error,
        _ => return Err(LogError::LevelDesconhecido), 
    };

    Ok(LogEntry {
        level,
        timestamp,
        message,
    })
}

fn read_logs_files(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut valid_logs : Vec<LogEntry> = Vec::new();
    let mut parsing_errors = 0;

    for result_line in reader.lines() {
       let line = result_line?;

        match parse_log_line(&line){
            Ok(log) => {
                valid_logs.push(log);
            },            
            Err(e) => {
                parsing_errors += 1;
            } 
        }
    }

    println!("--- RELATÓRIO DE PROCESSAMENTO ---");
    println!("Total de logs lidos com sucesso: {}", valid_logs.len());
    println!("Linhas corrompidas ignoradas: {}", parsing_errors);

    let mut errors = 0;
    for l in &valid_logs{
        if l.level == LogLevel::Error { 
            errors += 1;
        }
    }
    
    println!("Total de Erros críticos encontrados: {}", errors);
    
    Ok(())
}

fn main() {
    if let Err(erro) = read_logs_files("logs.txt") {
        println!("Erro ao abrir o arquivo: {:?}", erro);
    }
}
