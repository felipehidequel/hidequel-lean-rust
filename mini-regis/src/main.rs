use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};

struct ItemBanco {
    valor: String,
    criado_em: Instant,
    ttl: Option<Duration>,
}

struct BancoKV {
    mapa: HashMap<String, ItemBanco>,
}

impl BancoKV {
    fn new() -> Self {
        BancoKV {
            mapa: HashMap::new(),
        }
    }

    fn set(&mut self, chave: String, valor: String, segundos_vida: Option<u64>) {
        let ttl = segundos_vida.map(Duration::from_secs);

        let item = ItemBanco {
            valor,
            criado_em: Instant::now(),
            ttl,
        };

        self.mapa.insert(chave, item);
    }

    fn get(&mut self, chave: &str) -> Option<&String> {
        let mut expirou = false;

        if let Some(item) = self.mapa.get(chave) {
            if let Some(tempo_limite) = item.ttl {
                if item.criado_em.elapsed() > tempo_limite {
                    expirou = true;
                }
            }
        } else {
            return None;
        }

        if expirou {
            self.mapa.remove(chave);
            return None;
        }

        self.mapa.get(chave).map(|item| &item.valor)
    }
}

fn command_execute(banco: &mut BancoKV, linha: &str) {
    let parts: Vec<&str> = linha.trim().split(' ').collect();

    let command = match parts.get(0) {
        Some(&cmd) => cmd.to_uppercase(),
        None => return,
    };

    match command.as_str() {
        "SET" => {
            if let (Some(&chave), Some(&valor)) = (parts.get(1), parts.get(2)) {
                let ttl = parts.get(3).and_then(|s| s.parse::<u64>().ok());

                banco.set(String::from(chave), String::from(valor), ttl);
                println!("Ok!");
            } else {
                println!("Erro: Uso correto -> SET <chave> <valor> [segundos]");
            }
        }
        "GET" => {
            if let Some(&chave) = parts.get(1) {
                match banco.get(chave) {
                    Some(v) => {
                        println!("{v}");
                    }
                    None => {
                        println!("(nil)")
                    }
                };
            }
        }
        "EXIT" => {
            println!("Saindo do mini-regis... Até mais!");
            std::process::exit(0);
        }
        _ => println!("Erro: Comando desconhecido '{}'", command),
    }
}

fn main() {
    let mut banco = BancoKV::new();
    println!("=== MINI-REGIS INTERATIVO ===");
    println!("Comandos aceitos: SET, GET, EXIT\n");

    loop {
        print!("mini-regis> ");
        io::stdout().flush().unwrap(); // Força o texto acima a aparecer no terminal imediatamente

        let mut linha = String::new();
        io::stdin().read_line(&mut linha).unwrap();

        command_execute(&mut banco, &linha);
    }
}
