use std::io;


fn main() {
    let stdin = io::stdin();
    let mut var = String::new();
    stdin.read_line(&mut var).unwrap();
    let lista: Vec<i32> = var.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let a:i32 = lista[0] + lista[1];
    println!("{:}",a); 
}
