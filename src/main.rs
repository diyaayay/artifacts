use colored::*;
use prettytable::{Table, row};
use std::thread;
use std::time::Duration;

fn print_colored_table(messages: Vec<(&str, Color)>) {
    let mut table = Table::new();
    table.add_row(row!["Message"]);

    for (message, color) in messages {
        let colored_message = message.color(color).to_string();
        table.add_row(row![colored_message]);
        thread::sleep(Duration::from_millis(500));
    }

    // Print the table at the end
    table.printstd();
    println!("{}", "HAVE A WONDERFUL DAY! :)))".bold().underline());
}

fn main() {
    let messages = vec![
        ("Hello, pr05 grantees!", Color::Blue),
        ("This is a colorful Rust program!", Color::Green),
        ("Enjoy the colors!", Color::Red),
        ("Stay safe, stay happy!", Color::Magenta),
        ("Rust is fun but I am very new to it!", Color::Yellow),
        ("Let's build something amazing during our time at the pr05!", Color::Cyan),
    ];

    print_colored_table(messages);
}
