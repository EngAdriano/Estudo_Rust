/* Blocos e Sombreamento  [3.1. Variables and Mutability] 
Baseado em:
The Rust Programming Language, 2nd Edition
by Steve Klabnik and Carol Nichols with contributions from the Rust Community
Copyright 2018 The Rust Project Developers. See the COPYRIGHT file at the top-level directory of this distribution and at
http://rust-lang.org/COPYRIGHT.
*/

fn main() {
    println!("Início do programa");
    const X: i32 = 5;
    let y: i32 = 6;
    let mut z: i32 = 7;
    z += 1;
    println!("No início do programa, o valor de X é: {}, o valor de y é: {}, o valor de z é: {}", X, y, z);

    {
        const X: i32 = 555;
        let y: i32 = 666;
        let mut z: i32 = 777;
        z += 1;
        println!("Dentro do bloco, o valor de X é: {}, o valor de y é: {}, o valor de z é: {}", X, y, z);
    }

    println!("No final do programa, o valor de X é: {}, o valor de y é: {}, o valor de z é: {}", X, y, z);

    // Segunda parte do programa
    // -------------------------------------------------------------------------------------------------------------------------------
    let a: i32 = 5;                     // A variável a é declarada e inicializada com o valor 5
    println!("O valor de a é: {}", a);
    let a: i32 = a + 1;                 // Sombreamento de variável: a variável a é declarada novamente, e seu valor é o valor anterior de a mais 1
    println!("O valor de a é: {}", a);

    {
        let a: i32 = a * 2;     // Sombreamento de variável: a variável a é declarada novamente dentro do bloco, e seu valor é o valor anterior de a multiplicado por 2 
        println!("O valor de a no bloco interno é: {}", a);
    }

    println!(" O valor de a depois do bloco interno é: {}", a);

    let spaces: &str = "   ";
    println!("O valor de spaces é: '{}'", spaces);
    let spaces: usize = spaces.len();                   // Sombreamento de variável: a variável spaces agora é do tipo usize, e não mais do tipo &str   
    println!("O valor de spaces é: '{}'", spaces);

    let mut spaces2: &str = "   ";
    println!("O valor de spaces2 é: '{}'", spaces2);
    spaces2 = "qwerty";                                // A variável spaces2 é mutável, então podemos alterar seu valor, mas não seu tipo
    println!("O valor de spaces2 é: '{}'", spaces2);

    // spaces2 = spaces2.len();                             // Erro de compilação: não podemos atribuir um valor do tipo usize a uma variável do tipo &str
    
}
