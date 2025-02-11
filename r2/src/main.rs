use std::io;
fn main() {
    let mut cadena = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut cadena).unwrap();
    let lista:Vec<i16> = cadena.trim().split_whitespace().map(|x| x.parse::<i16>().unwrap()).collect();
    let s = (lista[1]<<1) - lista[0];
    println!("{}",s);
}
