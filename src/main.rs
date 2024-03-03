const MEU_NOME: i8 = 120;

fn my_name(nome: &str) -> String {
    format!("{} Ramiro 😍", nome)
}

// não precisa do "return" e ";"
// A ultima expressão é o retorno da função
//     e precisa bater com o tipo de retorno da função
fn add_numbers(x: u32, y: u32) -> u32 {
    if x == 0 {
        return y; // o "return" existe para isso: retornar antecipadamente
    };
    // esse eh o retorno principa da função
    x + y
}

fn criar_nome() -> String {
    String::from("Marcos")
}

fn convert_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

fn double(n: i32) -> i32 {
    2 * n
}

// é necessário "importar" a trait, pois o comportamento está nela
mod animais; // aqui é feita a importação do módulo
use animais::{Cachorro, FazerSom, Gato}; // aqui declaro o que vou usar do módulo
//use animais::teste::msg_teste;// cargo watch -x run  -- hot reload
use rand::prelude::random;

fn to_Gato(i: i32) -> Gato {
    Gato::novo("nome", i)
}

fn maior<'a>(nome1: &'a str, nome2: &'a str) -> &'a str {
    if nome1.len() > nome2.len() {
        &nome1
    } else {
        &nome2
    }
}

fn teste_lifetime<'a>(nome1: &'a str) -> &'a str {
    
    let mut input2 = String::new();
    println!("Digite o segundo nome:");
    io::stdin()
    .read_line(&mut input2)
    .expect("Falha ao ler a linha");

    let resultado = maior(&nome1, &input2);
    resultado
}

use std::io;

fn main() {

    println!("Digite o primeiro nome:");
    let mut input = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("Falha ao ler a linha");


    let resultado = teste_lifetime(&input);
    println!("O resultado é {}", resultado);

    let xana = Gato::novo("Xana", 2);
    println!("{}", xana.fazer_som());

    println!("meu numero é: {}", random::<u8>());
    

    // Funções
    // snack_case -> exemplo: fn falar_ola_mundo() {}
    // expressões
    // algo que produz valor
    // statement - comandos -> não produz um valor de retorno

    let input = "1 2 3 4 5 6 7 8 9 10";

    // função anonima ou closure
    let resultado: Vec<Gato> = input
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .map(|n| n * 2)
        .map(to_Gato)
        .collect();

    println!(">> array de numeros: {:?}", resultado);
    for x in resultado {
        println!("iterador: {:?}", x);
    }


    let resultado: Vec<i32> = input
        .split(" ")
        .map(convert_to_number)
        .map(double)
        .collect();

    println!("Meu array de numeros: {:?}", resultado);

    let resultado = input.split(" ");
    println!("{:?}", resultado);

    println!("X + Y = {}", add_numbers(5, 6));
    println!("Meu nome eh {}", criar_nome());

    // () -> isso é uma expressão - unit type - em valor de retorno - rust não tem "null"
    let x = {
        let nome = "Marcos";
        // caso eu coloquei uma expressao aqui, será o retorno dessa "função"
        nome
    };
    println!("O resultado é: {}", x);

    // Struct
    let xana = Gato::novo("Xana", 2);
    println!("{}", xana.fazer_som());

    let max = Cachorro::novo("Max", 4);
    println!("{}", max.fazer_som());

    // String -> essa á a String de verdade
    // essa vive na HEAP
    let nome: String = String::from("Marcos");
    let nome_completo: String = my_name(&nome);
    let nome_completo_novo: String = "Marcos Ramiro".into(); // tem uma trait que faz isso para nós.

    println!("Meu nome é {}", nome);
    println!("Meu nome completo é {}", nome_completo);
    println!(
        "O tamnho do nome é {}",
        nome_completo.trim().chars().count()
    );

    // &str | slice string | str reference -> fica na stack
    // variavel vive na STACK e a literal na STATIC
    let nome: &str = "Marcos\n";
    println!("Meu nome é {}", nome);
    println!("Tamanho da string {}", nome.trim().len());
    println!("Tamanho da string {}", nome.trim().chars().count());
    // ** Tipos primitivos "Compostos" **

    //array
    let mut my_arr: [i32; 6] = [1, 2, 3, 5, 0, 8];
    my_arr[0] = 50;
    my_arr = [100, 200, 300, 400, 500, 600];
    println!("numero array: {}", my_arr[0]);
    println!("array: {:?}", my_arr);
    println!("tamanho array: {}", my_arr.len());

    // slice
    let novo_arr: &[i32] = &my_arr[2..];
    println!("tamanho novo array: {}", novo_arr.len());

    println!("slice: {:?}", &my_arr[1..2]);

    // tupla
    let mut numeros: (i32, i32, f64) = (1, 2, 3.789);
    let (a, b, c) = numeros;
    numeros.0 = 50;
    numeros = (100, 200, 300.1);
    println!("Numeros: {:?}", numeros);
    println!("Numero 0: {}", a);
    println!("Numero 2: {}", numeros.2);

    // ** Tipos primitivos Escalares **
    let meu_nome = "Marcos";
    let carac: char = '🙌';
    let valor: i64 = 56_598_456_454;
    let my_float: f64 = 45.78;
    let my_boo = true;
    let my_boo: bool = false;
    let letra: char = 'a'; // até 4 bytes - utf8

    {
        let meu_nome = "Maria";
        println!("Valor {} <-> {}", valor, meu_nome);
    }

    println!("Valor {} <-> {} || {}", minha_fun(valor), meu_nome, carac);
}

fn minha_fun(valor: i64) -> i64 {
    print!("Entrou aqui");
    return valor;
}
