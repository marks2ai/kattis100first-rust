use std::io;
fn main() {
    let stdin = io::stdin();
    let mut cadena = String::new();
    let mut nuevo: String = String::new();
    stdin.read_line(&mut cadena).unwrap();
    let mut c = 0;
    for x in cadena.chars() {
        if x != 'e'{
            nuevo.push_str(&String::from(x));
        }
        else {
            nuevo.push_str("ee");
        }
    }
    println!("{}", nuevo);
    
}
