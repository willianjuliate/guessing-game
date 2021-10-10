use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::Command;

fn main() {
    println!("###  Adivinhe o número!  ###");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {} ", secret_number);

    loop {
        println!("Por favor, insira seu palpite.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha!"); // I/O Input and Output Keyboard

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            }
        }
    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
