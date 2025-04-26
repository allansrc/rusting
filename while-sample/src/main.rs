use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "exit" {
        input.clear();
        println!("Please enter a words or command (or 'exit' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input);
    }
    println!("Exiting the program.");
}
