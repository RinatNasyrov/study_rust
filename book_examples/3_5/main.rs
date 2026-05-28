use std::io;

// 1. Конвертация температур между значениями по Фаренгейту к Цельсию.
// 2. Генерирование n-го числа Фибоначчи.
// 3. Распечатайте текст рождественской песни "Двенадцать дней Рождества", воспользовавшись повторами в песне.

fn main() {
    // На неиспользуемое выдает варнинги
    // convert_fr_to_c();
    // calc_fib_input();
    xmas_carol();
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

fn count_suffix(number: usize) -> String {
    match number {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        other => "th".to_string(),
    }
}

fn xmas_carol() {
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    const LEN: usize = 12;
    let mut day_counter: usize = 0;

    while day_counter < LEN {
        println!("On the {}{} day of Christmas,", day_counter + 1, count_suffix(day_counter + 1));
        println!("my true love sent to me");
        
        for gift_counter in (0..day_counter + 1).rev() {
            if gift_counter == 0 {
                if day_counter == 0 {
                    println!("A {}", gifts[gift_counter]);
                }
                else {
                    println!("And a {}", gifts[gift_counter]);
                }
            }
            else {
                println!("{} {}", gift_counter + 1, gifts[gift_counter]);
            }
        } 

        println!("");

        day_counter += 1;
    } 
}