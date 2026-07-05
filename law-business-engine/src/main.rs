use std::collections::HashMap;



#[derive(Debug)]
enum Expressao {
    Variavel(String),

    // pure boolean value
    Constante(bool),

    // NOT "!" operation
    Nao(Box<Expressao>),

    // AND operation
    E(Box<Expressao>, Box<Expressao>),

    // OR operation
    Ou(Box<Expressao>, Box<Expressao>),
}

impl Expressao {
    fn avaliar(&self, context: &HashMap<String, bool>) -> bool {
        match self {
            Expressao::Constante(val) => *val,
            Expressao::Nao(expr_interna) => !expr_interna.avaliar(&context),
            Expressao::Ou(esquerda, direita) => esquerda.avaliar(&context) || direita.avaliar(&context),
            Expressao::E(esquerda, direita) => esquerda.avaliar(&context) && direita.avaliar(&context),

            Expressao::Variavel(nome_var) => {
                context.get(nome_var).copied().unwrap_or(false)
            }
        }
    }
}

fn main() {
    // Regra de negócio: O cliente ganha frete grátis se:
    // (cupom_valido OU eh_vip) E (NAO frete_internacional)
    let regra_frete_gratis = Expressao::E(
        Box::new(Expressao::Ou(
            Box::new(Expressao::Variavel(String::from("cupom_valido"))),
            Box::new(Expressao::Variavel(String::from("eh_vip"))),
        )),
        Box::new(Expressao::Nao(
            Box::new(Expressao::Variavel(String::from("frete_internacional")))
        )),
    );

    // Simulando os dados da requisição atual do cliente
    let mut contexto_cliente = HashMap::new();
    contexto_cliente.insert(String::from("cupom_valido"), false);
    contexto_cliente.insert(String::from("eh_vip"), true);
    contexto_cliente.insert(String::from("frete_internacional"), false);

    let resultado = regra_frete_gratis.avaliar(&contexto_cliente);
    println!("Cliente ganhou frete grátis? {}", resultado);
}