/* Variáveis e Mutabilidade [3.1. Variables and Mutability] 

Baseado em:
The Rust Programming Language, 2nd Edition
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
*/

const MAX_POINTS: u32 = 100000; // Constantes são imutáveis e devem ter um tipo explícito
const UMA_HORA_EM_SEGUNDOS: u32 = 60 * 60; // Constantes podem ser usadas para expressões
const PI: f64 = 3.141592653589793; // Constantes podem ser de qualquer tipo escalar

fn main() {
    println!("Início do programa");
    let x: i32 = 5; // Imutável por padrão
    println!("O valor de x é: {}", x);

    // x = 6; // Isso causará um erro de compilação, pois x é imutável

    let x: i32 = 6; // Shadowing (sombreamento) - podemos declarar uma nova variável com o mesmo nome
    println!("O valor de x após shadowing é: {}", x);

    let mut y: i32 = 10; // Variável mutável
    println!("O valor de y é: {}", y);
    y = 15; // Agora podemos modificar y, pois é mutável
    println!("O valor de y após modificação é: {}", y);

    let mut z: i32 = 20; // Variável mutável com tipo explícito
    let mut w: i32 = 30; // Variável mutável com tipo explícito e palavra-chave mut
    let mut a: f64 = 40.00; // Variável mutável com tipo explícito e palavra-chave mut
    println!("O valor de z é: {}", z);
    println!("O valor de w é: {}", w);
    println!("O valor de a é: {}", a);

    z = MAX_POINTS as i32; // Atribuindo o valor da constante MAX_POINTS a z
    println!("O valor de z após atribuir MAX_POINTS é: {}", z);
    w = UMA_HORA_EM_SEGUNDOS as i32; // Atribuindo o valor da constante UMA_HORA_EM_SEGUNDOS a w
    println!("O valor de w após atribuir UMA_HORA_EM_SEGUNDOS é: {}", w);
    a = PI as f64; // Atribuindo o valor da constante PI a a
    println!("O valor de a após atribuir PI é: {}", a);

    println!("Fim do programa");
}
