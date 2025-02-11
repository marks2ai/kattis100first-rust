use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let mut n:u64 = cad.trim().parse::<u64>().unwrap();
    n = if n&1 == 0 { (n +1) >> 1 } else{ n >> 1 };
    println!("{}",n);
}
