use std::io;

fn main() {
    // На неиспользуемое выдает варнинги
    // convert_fr_to_c();
    // calc_fib_input();
}

fn convert_fr_to_c() {
    println!("Введите температуру в Фаренгейтах: ");
    
    let mut farenheit = String::new();
    
    io::stdin()
        .read_line(&mut farenheit)
        .expect("bad input");

    let farenheit: f64 = farenheit
        .trim()
        .parse()
        .expect("bullshit input");

    let celsius = (farenheit - 32.0) * 5.0 / 9.0;

    println!("Температура в Цельсиях: {celsius}");
}

fn calc_fib_input() {
    println!("Введите номер числа: ");
    
    let mut num = String::new();
    
    io::stdin()
        .read_line(&mut num)
        .expect("bad input");

    let num: u32 = num
        .trim()
        .parse()
        .expect("bullshit input");

    let fib_num = calc_fib(num);    
    println!("Число Фибоначчи: {fib_num}");
}

fn calc_fib(num: u32) -> u32 {
    match num {
        0 => return 0,
        1 => return 1,
        other => return calc_fib(other - 1) + calc_fib(other - 2),
    }
}