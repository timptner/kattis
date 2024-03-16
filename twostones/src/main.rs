use std::io::stdin;

fn main() {
    let input = get_input();
    let number: i32 = input[0].parse().expect("not a number");
    let remainder: i32 = number % 2;
    match remainder {
        0 => println!("Bob"),
        1 => println!("Alice"),
        _ => panic!("Unknown value"),
    }
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
