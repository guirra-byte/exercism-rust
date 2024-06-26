use std::io;

fn convert_to_int(input: &String) -> u32 {
    let data = input.trim().parse::<u32>().unwrap();
    return data;
}

fn diff() {
    let mut left = String::new();
    println!("Digite um número:");
    io::stdin().read_line(&mut left).expect("Error ao ler left");

    let mut right = String::new();
    println!("Digite outro número:");
    io::stdin()
        .read_line(&mut right)
        .expect("Error ao ler right");

    if left != "" && right != "" {
        let (left_number, right_number) = (convert_to_int(&left), convert_to_int(&right));
        println!(
            "{} + {} = {}",
            left_number,
            right_number,
            (left_number + right_number)
        );
    }
}

fn sum() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error ao ler input");
    let input_chars = input.chars().into_iter();
    let mut result: u32 = 0;
    for char in input_chars {
        if &char.to_string() != "\n" {
            let convert = convert_to_int(&char.to_string());
            result += convert;
        }
    }

    println!("{}", result);
}

pub fn main() {
    sum();
    diff();
}
