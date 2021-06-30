use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("ドドンドドンドン  ドン  かずあてぺこ！");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("予想をいれるぺこ？");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("入力に問題があったぺこ…");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あんたの予想はこれ: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎるぺこ！やり直し！"),
            Ordering::Greater => println!("大きすぎるぺこ！やり直し！"),
            Ordering::Equal => {
                println!("正解ぺこ！");
                break;
            }
        }
    }

    println!("")
}
