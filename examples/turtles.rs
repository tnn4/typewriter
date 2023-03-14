use colored::Colorize;

fn main() {

    println!("starting sleep...");
    terminal::mssleep(50);
    println!("finished sleep");

    terminal::term::type_text("Helllllllllo world!\n");

    terminal::term::type_text("Could you tell me what the world sits upon?\n");

    terminal::term::type_color("A turtle.\n".green());

    "this is blue".blue();

    terminal::term::type_text_colored("red","some stringggggggggggggggggggggggggggggggggg");
    terminal::term::type_text_colored("blue","some stringggggggggggggggggggggggggggggggggg");
    terminal::term::type_text_colored("green","some stringggggggggggggggggggggggggggggggggg");

}