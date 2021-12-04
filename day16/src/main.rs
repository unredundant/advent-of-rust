use std::fs;

fn main() {
    let contents = read_file();
    println!("{}", contents);
}

fn read_file() -> String {
    let filename = "input_data.txt";
    fs::read_to_string(filename).expect("Unable to read file")
}