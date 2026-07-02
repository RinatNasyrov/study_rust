/*
 * 1. Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка:
 * среднее значение; медиану (значение элемента из середины списка после его сортировки);
 * моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз;
 * HashMap будет полезна в данном случае).
 * 2. Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin). Первая согласная каждого
 * слова перемещается в конец и к ней добавляется окончание "ay", так "first" станет "irst-fay".
 * Слову, начинающемуся на гласную, в конец добавляется "hay" ("apple" становится "apple-hay").
 * Помните о деталях работы с кодировкой UTF-8!
 * 3. Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю
 * добавлять имена сотрудников к названию отдела компании. Например, "Add Sally to Engineering"
 * или "Add Amir to Sales". Затем позвольте пользователю получить список всех людей из отдела
 * или всех людей в компании, отсортированных по отделам в алфавитном порядке.
 */
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    first_task();
    second_task();
}

fn second_task() {
    // 2. Работаем со строкой
    let text = "An big elephant can eat ten apples daily";
    println!("Исходная фраза {text}");

    let mut res = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for word in text.split_whitespace() {
        if word.len() == 1 {
            continue;
        }

        // Разное поведение в зависимости от гласности
        if vowels.iter().any(|x| word.to_lowercase().starts_with(*x)) {
            // Разложим слово на символы
            let mut chars_vec: Vec<_> = word.chars().collect();
            // Достанем первый символ из начала в суффикс в конце
            let char: char = chars_vec.remove(0);

            let trimmed_word: String = chars_vec.into_iter().collect();
            res.push_str(&trimmed_word);
            res.push('-');
            res.push(char);
            res.push_str("ay ");
        } else {
            res.push_str(word);
            res.push_str("-hay ");
        }
    }

    println!("Измененная фраза {res}");
}

fn first_task() {
    // 1. Работаем с вектором целых чисел
    let mut nums: Vec<u8> = Vec::new();
    const COUNT: u8 = 11;
    let mut i: u8 = 0;
    while i < COUNT {
        i += 1;
        let value: u8 = rand::rng().random_range(1..=10);
        nums.push(value);
    }

    println!("Исходный вектор");
    // Похоже просто передать дважды вектор нельзя
    // линтер указывает на то, что nums уже был перемещен,
    // что круто тк передавать копию очевидно безопаснее
    print_vec(&nums);
    let avg = calc_vec_avg(&nums);

    println!("\nСреднее {avg}");
    sort_vec(&mut nums);

    println!("Сортируем");
    print_vec(&nums);

    let median = find_median(&nums);
    println!("\nМедиана {median}");

    let moda = find_moda(&nums);
    println!("Мода {moda}");
}

fn print_vec(v: &Vec<u8>) {
    for i in v {
        print!("{i} ");
    }
}

fn calc_vec_avg(v: &Vec<u8>) -> usize {
    let mut sum: u8 = 0;
    for i in v {
        sum += i;
    }
    sum as usize / v.len()
}

fn sort_vec(v: &mut Vec<u8>) {
    // Пузырек
    let mut buff: u8;
    // Проигнорирую авто-разадресацию для себя для наглядности
    for i in 0..(*v).len() {
        let swapped = false;
        for j in 0..(*v).len() - i - 1 {
            if v[j] > v[j + 1] {
                buff = v[j];
                v[j] = v[j + 1];
                v[j + 1] = buff;
            }
        }
        if swapped {
            break;
        }
    }
}

fn find_median(v: &Vec<u8>) -> f32 {
    // Добавил доп условие для решения ситуаций с
    // нечетным колвом элементов тк такое было когда то в унике
    let n: usize = v.len();
    let mut res: f32;
    if n % 2 == 0 {
        res = v[n / 2] as f32;
    } else {
        res = (v[n / 2] + v[n / 2 - 1]) as f32;
        res = res / 2.0;
    }
    res
}

fn find_moda(v: &Vec<u8>) -> i8 {
    let mut map = HashMap::new();
    let mut frequently_seen: i8 = -1;
    let mut max_count = 0;

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            frequently_seen = *i as i8;
        }
    }

    frequently_seen
}
