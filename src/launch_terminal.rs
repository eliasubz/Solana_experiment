use std::io::{self, Write};
use std::process::Command;

pub fn launch_new_terminal() -> io::Result<()> {
    let os = std::env::consts::OS;

    let (terminal, args) = match os {
        "windows" => ("cmd", vec!["/C", "start", "cmd", "/K"]),
        "macos" => (
            "osascript",
            vec!["-e", r#"tell app "Terminal" to do script ""#],
        ),
        "linux" => ("x-terminal-emulator", vec!["-e"]),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported operating system",
            ))
        }
    };

    let initial_commands = vec![
        "echo Welcome to the Solana Transaction Reader!",
        "echo Type 'exit' to close this window.",
        // Add more commands here as needed
    ];

    let full_command = match os {
        "windows" => {
            let commands = initial_commands.join(" && ");
            // args.into_iter().chain(vec![commands]).collect()
        }
        "macos" => {
            let commands = initial_commands.join("; ");
            let script = format!(r#"tell app "Terminal" to do script "{}""#, commands);
            // vec!["-e", &script]
        }
        "linux" => {
            let commands = initial_commands.join("; ");
            // vec!["-e", &format!("bash -c '{}'", commands)]
        }
        _ => unreachable!(),
    };

    Command::new(terminal); //.args(full_command).spawn()?;
    println!("New terminal window opened successfully");
    Ok(())
}
