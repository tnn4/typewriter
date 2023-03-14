use colored::Colorize;

fn main() {

    println!("starting sleep...");
    terminal::mssleep(50);
    println!("finished sleep");

    terminal::term::type_text("Helllllllllo world!\n");

    terminal::term::type_text("Could you tell me what the world sits upon?\n");

    for i in 0..10 {
        terminal::term::type_text_colored("red","A turtle");
        terminal::term::type_text("And that sits upon?\n");
    }
    terminal::term::type_text_colored("red", "A TURTLE! THERE ARE TURTLES ALL THE WAY DOWN");

    terminal::term::type_text_colored("blue","some stringggggggggggggggggggggggggggggggggg");
    terminal::term::type_text_colored("green","some stringggggggggggggggggggggggggggggggggg");

}