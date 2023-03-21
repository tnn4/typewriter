use std::{thread, time};


use colored::Colorize;

use typewriter::term::type_text_colored;
use typewriter::clear_console;

fn main() {
    type_text_colored("red", "Forge World Graia");
    type_text_colored("red", "Industrial output: Warlord Class Titan");
    println!("{}","Strategic value absolute".red().blink());
    let blinky_red = "BLINKING RED".red().blink();
    type_text_colored("oj", "Xenos invasion in progress");
    type_text_colored("oj", "Recommended course of action?");
    type_text_colored("red", "Negative, strategic value absolute");
    
    type_text_colored("oj", "Deploy capital weaponry?");
    type_text_colored("red", "negative estimated reduction in manufacturing output unacceptable");
    
    type_text_colored("oj", "Liberation fleet?");

    println!("{}","Affirmative. Minor Elements in System. ETA = 5-37 days".red());
    type_text_colored("oj", "Delay unacceptable");

    type_text_colored("red", "Loss of strategic assets on GRAIA unacceptable.");
    println!("{}","Strategic value ABSOLUTE".red().blink());

    type_text_colored("oj", "Escalate area of denial?");
    type_text_colored("red", "Affirmative");
    type_text_colored("oj", "Execute request order");
    type_text_colored("oj", "ADEPTUS ASTARTES ULTRA");
    println!("{}","Response incoming".truecolor(255,165,0).blink());
    mssleep(2000);
    type_text_colored("oj", "Deploying the Ultramarines.")


}

fn mssleep(ms: u64) {
    let milliseconds = time::Duration::from_millis(ms);
    thread::sleep(milliseconds);
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
