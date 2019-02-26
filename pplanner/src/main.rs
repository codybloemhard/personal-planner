use std::io;
use std::io::Write; //flush stdout

fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    inp = inp.trim().to_string();
    return inp;
}

fn prompt(msg : &str) -> String {
    print!("{}", msg);
    io::stdout().flush().ok().expect("Error: stdout().flush() in fn prompt");
    return read_inp();
}

fn receive_command() {
    loop{
        let x = prompt("cmd > ");
        match x.as_ref() {
            "q" => {break;},
            "quit" => {break;},
            _ => println!("Command [\"{}\"] not found!", x)
        }
    }
}

fn main() {
    println!("Personal Planner");
    receive_command();
}
