use std::time::Duration;
use tokio::{join, time::sleep};

async fn buscar_metrica_servidor(nome: &str, delay_segundos: u64) -> Result<u32, String> {
    sleep(Duration::from_secs(delay_segundos)).await;

    if nome == "Servidor_C" {
        return Err(String::from("Timeout ao conectar ao Servidor C"));
    }

    Ok(42 + delay_segundos as u32)
}

fn resultado(var: Result<u32, String>, nome: &str) {
    match var {
        Ok(value) => {
            println!("Uso de CPU Servidor {nome}: {value}");
        }
        Err(e) => {
            println!("ERRO: {e}");
        }
    };
}

#[tokio::main]
async fn main() {
    println!("=== COLETOR DE MÉTRICAS ASSÍNCRONO ===");

    let future_a = buscar_metrica_servidor("Servidor_A", 1);
    let future_b = buscar_metrica_servidor("Servidor_B", 2);
    let future_c = buscar_metrica_servidor("Servidor_C", 1);

    println!("Disparando todas as coletas em paralelo...");

    let (res_a, res_b, res_c) = join!(future_a, future_b, future_c);

    resultado(res_a, "A");
    resultado(res_b, "B");
    resultado(res_c, "C");
}
