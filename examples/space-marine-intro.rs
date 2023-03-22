use std::{thread, time};


use colored::Colorize;

use typewriter::term::typewriterc;
use typewriter::clear_console;

fn main() {
    typewriterc("red", "Forge World Graia\n");
    typewriterc("red", "Industrial output: Warlord Class Titan\n");
    println!("{}","Strategic value ABSOLUTE".red().blink());
    let blinky_red = "BLINKING RED".red().blink();
    typewriterc("oj", "Xenos invasion in progress");
    typewriterc("oj", "Recommended course of action?");
    typewriterc("oj", "Exterminatus?");
    typewriterc("red", "Negative, strategic value ABSOLUTE");
    
    typewriterc("oj", "Deploy capital weaponry?");
    typewriterc("red", "negative estimated reduction in manufacturing output unacceptable");
    
    typewriterc("oj", "Liberation fleet?");

    println!("{}","Affirmative. Minor Elements in System. ETA = 5-37 days".red());
    typewriterc("oj", "Delay unacceptable");

    typewriterc("red", "Loss of strategic assets on GRAIA unacceptable.");
    println!("{}","Strategic value ABSOLUTE".red().blink());

    typewriterc("oj", "Escalate area of denial?");
    typewriterc("red", "Affirmative");
    typewriterc("oj", "Execute request order");
    typewriterc("oj", "ADEPTUS ASTARTES ULTRA");
    println!("{}","Response incoming".truecolor(255,165,0).blink());
    mssleep(2000);
    typewriterc("oj", "Deploying the Ultramarines.")


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
