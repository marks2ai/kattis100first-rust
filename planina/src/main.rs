use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cad = String::new();
    stdin.read_line(&mut cad).unwrap();
    let n:i32 =  cad.trim().parse::<i32>().unwrap();
    let mut s = 2;
    for i in 0..n{
        s = s*2 - 1;
    }
    s = s*s;
    println!("{}",s);
}
