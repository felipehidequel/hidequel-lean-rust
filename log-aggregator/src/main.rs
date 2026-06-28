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

fn main() {
    let log_exemplo = "2026-06-27 20:00:00 [ERROR] Falha ao se conectar ao banco";
    
    match parse_log_line(log_exemplo) {
        Ok(log) => println!("Log processado com sucesso: {:?}", log),
        Err(e) => println!("Erro ao processar linha: {:?}", e),
    }
}
