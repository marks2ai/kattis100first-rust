use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cadena = String::new();
    stdin.read_line(&mut cadena).unwrap();
    let lista:Vec<i32> = cadena.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let suma = lista[0]+lista[1];
    println!("{}", suma);
}
