use rand::Rng;
use std::io;
use std::cmp::Ordering;

const XX: u8 = 10;

pub(crate) fn basic_syntax() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    let arr = [10, 20, 30, 40, 50];
    for ele in arr.iter() {
        println!("the value is: {}", ele);
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    let cond_number = 5;
    let number = if cond_number < 10 { 1 } else { -1 };
    println!("The value of number is: {}", number);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "first ele: {}, second ele: {}, third ele: {}",
        tup.0, tup.1, tup.2
    );
    let duplicate = [3; 5];
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    println!("XX: {}", XX);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{} spaces.", spaces);

}

pub(crate) fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You are right!");
                break;
            }
        }
    }
}
