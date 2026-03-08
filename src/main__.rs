use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Welcome to the Solana Transaction Reader!");
    println!("Type 'exit' to quit the program.");

    loop {
        print!("> ");
        io::stdout().flush()?; // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input == "exit" {
            println!("Exiting the program. Goodbye!");
            break;
        }

        // Here you can add custom commands or just echo the input
        match input {
            "help" => {
                println!("Available commands:");
                println!("  help  - Show this help message");
                println!("  exit  - Exit the program");
                // Add more commands here as you develop your program
            }
            _ => println!("You entered: {}", input),
        }
    }

    Ok(())
}
