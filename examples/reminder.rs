use colored::Colorize;

use typewriter::term::type_text_colored;
use typewriter::clear_console;

fn main() {
    type_text_colored("red", "Urgent: Talk to the bank about maintenance fees.");
    let blinky_red = "BLINKING RED".red().blink();
    colored_example();
    println!("{}", blinky_red);
    clear_console();
    println!("hello!");
    print!("\r");
    print!("got erased");
}

fn colored_example() {
    "this is blue".blue();
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can use truecolor values too!".truecolor(0, 255, 136);
    "background truecolor also works :)".on_truecolor(135, 28, 167);
    "you can also make bold comments".bold();
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style".red().bold().clear();
    "purple and magenta are the same".purple().magenta();
    "bright colors are also allowed".bright_blue().on_bright_white();
    "you can specify color by string".color("blue").on_color("red");
    "and so are normal and clear".normal().clear();
    String::from("this also works!").green().bold();
    format!("{:30}", "format works as expected. This will be padded".blue());
    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
}

