use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let mut lista:Vec<u16> = cad.trim().split_whitespace().map(|x| x.parse::<u16>().unwrap()).collect();
    let r = lista[0]*lista[1] - (lista[0]-1);
    println!("{}",r);
}
