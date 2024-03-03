pub mod teste;

#[derive(Debug)]
pub struct Gato {
    nome: String,
    idade: i32,
}

pub struct Cachorro {
    nome: String,
    idade: i32,
}


impl Gato {
    pub fn novo(nome: &str, idade: i32) -> Gato {
        let novo_nome = String::from(nome);
        Gato {
            nome: novo_nome,
            idade: idade,
        }
    }
}

impl Cachorro {
    pub fn novo(nome: &str, idade: i32) -> Cachorro {
        let novo_nome = String::from(nome);

        Cachorro {
            nome: novo_nome,
            idade: idade,
        }
    }
}

pub trait FazerSom {
    fn fazer_som(&self) -> String;
}

impl FazerSom for Gato {
    fn fazer_som(&self) -> String {
        format!("{} tem {} ano(s) e faz MIAUUU...", self.nome, self.idade)
    }
}

impl FazerSom for Cachorro {
    fn fazer_som(&self) -> String {
        teste::msg_teste("Vou fazer o som de CACHORRO");
        format!("{} tem {} ano(s) e faz AUAU...", self.nome, self.idade)
    }
}
