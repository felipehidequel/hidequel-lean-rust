#[derive(Debug)]
enum Expressao {
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
    fn avaliar(&self) -> bool {
        match self {
            Expressao::Constante(val) => *val,
            Expressao::Nao(expr_interna) => !expr_interna.avaliar(),
            Expressao::Ou(esquerda, direita) => esquerda.avaliar() || direita.avaliar(),
            Expressao::E(esquerda, direita) => esquerda.avaliar() && direita.avaliar(),
        }
    }
}

fn main() {
    let regra = Expressao::E(
        Box::new(Expressao::Ou(
            Box::new(Expressao::Constante(true)),
            Box::new(Expressao::Constante(false)),
        )),
        Box::new(Expressao::Nao(
            Box::new(Expressao::Constante(false))
        )),
    );

    println!("Resultado da regra: {}", regra.avaliar());
}
