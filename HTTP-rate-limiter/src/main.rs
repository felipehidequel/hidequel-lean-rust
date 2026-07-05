use std::collections::HashMap;
use std::time::{Duration, Instant};

trait RateLimiter {
    fn request_verify(&mut self, ip: &str) -> bool;
}

struct ClientHistoric {
    request_made: u32,
    window_initialized_in: Instant,
}

struct FixedWindowLimiter {
    clients: HashMap<String, ClientHistoric>,
    max_limit: u32,
    window_duration: Duration,
}

impl FixedWindowLimiter {
    fn new(max_limit: u32, window_seconds: u64) -> Self {
        FixedWindowLimiter {
            clients: HashMap::new(),
            max_limit,
            window_duration: Duration::from_secs(window_seconds),
        }
    }

    fn update_limit(&mut self, new_limit: u32){
        self.max_limit = new_limit;
    }
}

impl RateLimiter for FixedWindowLimiter {
    fn request_verify(&mut self, ip: &str) -> bool {
        let agora = Instant::now();

        let historic = self
            .clients
            .entry(String::from(ip))
            .or_insert_with(|| ClientHistoric {
                request_made: 0,
                window_initialized_in: agora,
            });

        if historic.window_initialized_in.elapsed() > self.window_duration {
            historic.request_made = 0;
            historic.window_initialized_in = agora;
        }

        if historic.request_made < self.max_limit {
            historic.request_made += 1;
            return true;
        }
        return false;
    }
}

fn main() {
    let mut limiter = FixedWindowLimiter::new(1, 10);
    let ip = "192.168.1.100";

    println!("Requisição 1: {}", limiter.request_verify(ip)); 
    println!("Requisição 2 (Bloqueado): {}", limiter.request_verify(ip)); 

    println!("\n[Admin] Aumentando o limite dinamicamente para 3...");
    limiter.update_limit(3);

    println!("\n--- Testando novamente com novo limite ---");
    println!("Requisição 3: {}", limiter.request_verify(ip)); 
    println!("Requisição 4: {}", limiter.request_verify(ip)); 
    println!("Requisição 5 (Bloqueado de novo): {}", limiter.request_verify(ip));
}

