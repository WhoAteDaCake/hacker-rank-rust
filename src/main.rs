use std::io;

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let clean_input = String::from(input.trim());
    Ok(clean_input)
}

fn get_split_input(line: & String) -> std::str::SplitWhitespace {
    line.split_whitespace()
}

fn get_converted_line() -> Vec<i32> {
    let line = match get_input() {
        Ok(line) => line,
        _ => panic!("Could not get first line")
    };
    get_split_input(&line).map(| n | { n.parse::<i32>().unwrap() }).collect()
}

fn shift(list: & Vec<i32>, rotations: usize) -> Vec<i32> {
    let (left, right) = list.split_at(rotations);
    let concat = [&right[..], &left[..]].concat();
    Vec::from(concat)
}

fn main() {
    let first_line: Vec<i32> = get_converted_line();
    let second_line: Vec<i32> = get_converted_line();

    let n_count = first_line[0];
    let rotations = first_line[1] % n_count;
    let shifted_list = shift(&second_line, rotations as usize);
    // println!("");
    shifted_list.iter().for_each(| n | print!("{} ", n));
}