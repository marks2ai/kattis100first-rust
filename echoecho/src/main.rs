use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let limpio = cad.trim();
    println!("{} {} {}",limpio, limpio, limpio);

}
