use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");


    let mut guess = String::new() ;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let mut apple=56;
    println!("Number of apples are: {}",apple);
    let straw=900000;
    apple=78;
    println!("now it is {} and strwaberry is {}",apple,straw);
}
