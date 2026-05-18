fn main() {
    let mut input = String::new();
    println!("Digite um valor numérico");
    std::io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    
    let n: i32 = input.trim().parse().expect("Por favor, digite um número!");
    println!("Você digitou: {}", n);
}
