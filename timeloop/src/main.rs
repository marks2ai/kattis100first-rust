use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let n = cad.trim().parse::<u8>().unwrap();
    for i in 1..=n{
        println!("{}  Abracadabra",i);
    }
}
