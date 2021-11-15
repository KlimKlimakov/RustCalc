use std::io;

fn main() {
    println!("Этот калькулятор поддерживает только сложение");
    println!("И только два слогаемых");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut result = 0;

    println!("Первое число: ");
    io::stdin().read_line(&mut num1);

    println!("Второе число: ");
    io::stdin().read_line(&mut num2);

    let num1: u32 = num1.trim().parse().expect("Напишите внятно!");
    let num2: u32 = num2.trim().parse().expect("Напишите внятно!");

    result = num1 + num2;
    println!("{}", result)

}
