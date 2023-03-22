use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número");

    // Gera um número aleatório entre 1 e 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Imprime o número secreto para fins de teste
    println!("{secret_number}");

    // Loop até que o usuário adivinhe o número correto
    loop {
        // Lê o palpite do usuário da entrada padrão
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        // Converte o palpite do usuário em um número
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Imprime o palpite do usuário
        println!("Você adivinhou: {guess_number}");

        // Compara o palpite do usuário com o número secreto
        match guess_number.cmp(&secret_number) {
            Ordering::Greater => println!("Muito grande"),
            Ordering::Less => println!("Muito pequeno"),
            Ordering::Equal => {
                println!("Você ganhou");
                break;
            }
        }
    }
}