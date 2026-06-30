use std::collections::HashMap;
use std::time::{Instant, Duration};

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

    fn set(&mut self, chave: String, valor: String, segundos_vida: Option<u64>){
        let ttl = segundos_vida.map(Duration::from_secs);
        
        let item = ItemBanco {
            valor,
            criado_em: Instant::now(),
            ttl,
        };

        self.mapa.insert(chave,item);
    }

    fn get(&mut self, chave: &str) -> Option<&String> {
        let mut expirou = false;

        if let Some(item) = self.mapa.get(chave){
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

fn main() {
    let mut banco = BancoKV::new();

    banco.set(String::from("session_token"), String::from("XC3234"), Some(2));

    println!("Busca 1 (Imediata): {:?}", banco.get("session_token"));

    
    println!("Dormindo por 3 segundos...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Busca 2 (Após 3s): {:?}", banco.get("session_token"));
    println!("Quantidade de itens na memória: {}", banco.mapa.len());
}
