use std::env;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    println!("{:?}", file_name);
}
