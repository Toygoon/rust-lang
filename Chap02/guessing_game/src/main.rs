use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
