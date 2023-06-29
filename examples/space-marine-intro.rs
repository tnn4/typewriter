use std::{thread, time};


use colored::Colorize;

use tw::term::typewritec;
use tw::clear_console;
use tw::clear_line;
use tw::clear_line2;

fn main() {
    let blinky_red = "BLINKING RED".red().blink();
    // typewritec("red", "Forge World Graia\n");
    // typewritec("red", "Industrial output: Warlord Class Titan\r\n");
    // typewritec("red", "Industrial output: Warlord Class Titan\n\r");
    // typewritec("red", "Industrial output: Warlord Class Titan");
    // typewritec("red", "Industrial output: Warlord Class Titan\rhhklhklhklhlhklhlhlh\rhkjhlhlhlhhlhlh\rgjgjkgjggkgjkg");
    for i in 0..100 {
        typewritec("red","\r|\r/\r-\r\\\r");
    }
    typewritec("red","\r|\r/\r-\r\\\r");
    typewritec("red", "\rBack where it all started");
    typewritec("red", "hello");
    clear_line2();
    typewritec("red", "A.K.A BIG robot");
}

fn intro_40k(){
    typewritec("red", "Forge World Graia\n");
    typewritec("red", "Industrial output: Warlord Class Titan\n");
    println!("{}","Strategic value ABSOLUTE".red().blink());
    clear_console();
    typewritec("oj", "Xenos invasion in progress");
    typewritec("oj", "Recommended course of action?");
    typewritec("oj", "Exterminatus?");
    typewritec("red", "Negative, strategic value ABSOLUTE");
    clear_line();


    typewritec("oj", "Deploy capital weaponry?");
    typewritec("red", "negative estimated reduction in manufacturing output unacceptable");
    
    typewritec("oj", "Liberation fleet?");

    println!("{}","Affirmative. Minor Elements in System. ETA = 5-37 days".red());
    typewritec("oj", "Delay unacceptable");

    typewritec("red", "Loss of strategic assets on GRAIA unacceptable.");
    println!("{}","Strategic value ABSOLUTE".red().blink());

    typewritec("oj", "Escalate area of denial?");
    typewritec("red", "Affirmative");
    typewritec("oj", "Execute request order");
    typewritec("oj", "ADEPTUS ASTARTES ULTRA");
    println!("{}","Response incoming".truecolor(255,165,0).blink());
    mssleep(2000);
    typewritec("oj", "Deploying the Ultramarines.")
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
