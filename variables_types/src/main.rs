/*
/ Este código demonstra o uso de variáveis mutáveis e imutáveis em Rust.
/ Em Rust, por padrão, as variáveis são imutáveis. Para tornar uma variável mutável,
/ você deve usar a palavra-chave `mut`. Variáveis imutáveis não podem ser reatribuídas
/ depois de serem inicializadas, enquanto variáveis mutáveis podem ser reatribu
*/

const SECONDS_IN_MINUTE: u32 = 60; // constante, não pode ser alterada
const MINUTES_IN_HOUR: u32 = 60; // constante, não pode ser alterada
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR; // constante calculada a partir de outras constantes

fn main() {

    println!("O número de segundos em uma hora é: {}", SECONDS_IN_HOUR);

    let total: i32 = 30; // variável imutável
    let total_em_segundos: i32 = total * (SECONDS_IN_HOUR as i32); // total em segundos é calculado a partir de total
    println!("O total em segundos é: {}", total_em_segundos);


    let mut x: i32 = 5; // variável mutável
    println!("O valor de x é: {}", x);
    x = 6; // atribuição de um novo valor a x é permitida porque x é mutável
    println!("O valor de x é: {}", x);

    let y: i32 = 10; // immutable variable
    println!("O valor de y é: {}", y);
    // y = 11; // isto causará um erro de compilação porque y é imutável

    {
        let total: i32 = 50; // esta é uma nova variável total, que é diferente da variável total fora do bloco
        println!("O valor de total dentro do bloco é: {}", total);
    }
    
    println!("O valor de total fora do bloco é: {}", total); // isto imprimirá 30, não 50


}
