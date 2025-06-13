struct Veiculo {
    modelo: String,
    ano: u32,
    placa: String,
    cor: String,
}

trait Descricao {
    fn descricao(&self) -> String;
}

impl Descricao for Veiculo {
    fn descricao(&self) -> String {
        format!("Modelo: {}, Ano: {}, Placa: {}, Cor: {}", self.modelo, self.ano, self.placa, self.cor)
    }
}

fn main() {

    let mut modelo = String::new();
    let mut ano = String::new();
    let mut placa = String::new();
    let mut cor = String::new();

    println!("Monte seu veiculo ...");

    println!("Digite o modelo do veiculo: \n
    (Exemplo: Fusca, Gol, Palio, etc.) \n
    (Apenas o nome do modelo sem o ano, placa e cor)");
    std::io::stdin()
        .read_line(&mut modelo)
        .expect("Houve um erro na leitura dos dados");

    println!("Digite o ano do veiculo: \n
    (Exemplo: 1976, 2020, 2021, etc.) \n
    (Apenas o ano do veiculo)");
    std::io::stdin()
        .read_line(&mut ano)
        .expect("Houve um erro na leitura dos dados");

    println!("Digite a placa do veiculo: \n
    (Exemplo: ABC-1234, XYZ-5678, etc.) \n
    (Apenas a placa do veiculo)");
    std::io::stdin()
        .read_line(&mut placa)
        .expect("Houve um erro na leitura dos dados");

    println!("Digite a cor do veiculo: \n
    (Exemplo: Azul, Vermelho, Preto, etc.) \n
    (Apenas a cor do veiculo)");
    std::io::stdin()
        .read_line(&mut cor)
        .expect("Houve um erro na leitura dos dados");

    let veiculo = Veiculo {
        modelo: modelo.trim().to_string(),
        ano: ano.trim().parse().unwrap(),
        placa: placa.trim().to_string(),
        cor: cor.trim().to_string(),
    };

    println!("{}", veiculo.descricao());
}