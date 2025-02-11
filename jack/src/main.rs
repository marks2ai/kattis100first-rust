use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let lista:Vec<i32> = cad.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let res:i32 = lista.iter().product();
    println!("{}",res)
}
