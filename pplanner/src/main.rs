use std::io;

fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error reading line.");
    inp = inp.trim().to_string();
    return inp;
}

fn main() {
    println!("Henlo Frens!");
    loop{
        let x = read_inp();
        println!("{}", x);
        if x == "q".to_string() {
            break;
        }
    }
}
