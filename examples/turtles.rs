use colored::Colorize;

fn main() {

    println!("starting sleep...");
    terminal::mssleep(1000);
    println!("finished sleep");

    terminal::term::type_text("Helllllllllo world!\n");

    terminal::term::type_text("Could you tell me what the world sits upon?\n");

    terminal::term::type_text_colored("A turtle.\n".green());
}