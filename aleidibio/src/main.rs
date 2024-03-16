use std::io::stdin;

fn main() {
    let input = get_input();
    let numbers = parse_numbers(input);
    let sum = numbers[0] + numbers[1];
    let answer = numbers[2] - sum;
    println!("{answer}")
}

fn get_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    loop {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("failed to read stdin");
        let value = buffer.trim();
        if value == "" {
            break;
        }
        input.push(value.to_string());
    }
    dbg!(input)
}

fn parse_numbers(values: Vec<String>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for value in values {
        let number: i32 = match value.parse() {
            Ok(i) => i,
            Err(_) => {
                println!("not a number");
                continue;
            }
        };
        numbers.push(number);
    }
    numbers
}
