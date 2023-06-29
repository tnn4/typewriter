use colored::Colorize;

fn main() {

    const RED: &str = "red";

    println!("starting sleep...");
    tw::sleep_ms(50);
    println!("finished sleep");

    tw::term::typewritec(RED,"Helllllllllo world!\n");

    tw::term::typewritec(RED,"Could you tell me what the world sits upon?\n");

    for i in 0..10 {
        tw::term::typewritec(RED,"A turtle");
        tw::term::typewritec("green","And that sits upon?\n");
    }
    tw::term::typewritec("red", "A TURTLE! THERE ARE TURTLES ALL THE WAY DOWN");

    tw::term::typewritec("blue","some stringggggggggggggggggggggggggggggggggg");
    tw::term::typewritec("green","some stringggggggggggggggggggggggggggggggggg");

}