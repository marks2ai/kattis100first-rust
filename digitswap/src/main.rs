use std::io;
fn main() {
    let mut cadena = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut cadena).unwrap();
    let mut chars:Vec<char> = cadena.chars().collect();
    chars.swap(0, 1);
    cadena = chars.into_iter().collect();
    println!("{}",cadena);
}
