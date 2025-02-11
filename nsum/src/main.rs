use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    cad.clear();
    stdin.read_line(&mut cad).unwrap();
    let lista: Vec<u16> =cad.trim().split_whitespace().map(|x| x.parse::<u16>().unwrap()).collect();
    let suma: u16 = lista.into_iter().sum();
    println!("{}",suma);
}
