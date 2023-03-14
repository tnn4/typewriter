use std::{thread, time};

use std::fs;

use std::path::Path;


/** File Functions **/
fn read_file(path_to_file: &str) -> String{
    let contents = fs::read_to_string(path_to_file)
        .expect("Should be able to read the file."); // -> Result<String>
    contents
}

fn print_file_contents(input: &String){
    println!("File contents: {input}");
}

pub fn mssleep(ms_to_wait: u64)
{
    let time_to_wait = time::Duration::from_millis(ms_to_wait);
    thread::sleep(time_to_wait);
}

pub mod term {
    use std::io::{self, Write};

    use colored::Colorize;
    use colored::ColoredString;
    use colored::Color;

    /** Type out text**/
    // Should probably combine these functions by letting it take a trait object
    pub fn type_text(s: &str) 
    {
        let ms = 50;
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    
    pub fn type_text_in_ms(s: &str, ms: u64) 
    {
        
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }

    pub fn type_text_colored(s: ColoredString)
    {
        let sleep_time = std::time::Duration::from_millis(50);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Couldn't flush the terminal buffer!");
            std::thread::sleep(sleep_time);
        }
    }

    fn type_text_colored_in_ms(s: ColoredString, ms: u64) 
    {
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    

    /* Test fn*/
    fn analyze_colored_string(s: ColoredString)
    {
        let applied_color = s.fgcolor();
    
        match applied_color{
            Some(Color::Red) => println!("The color was RED!"),
            _ => println!("It was some other color or (None)"),
        }

        println!("Number of single character substrings");
        for n in 0..s.len()-1{
            print!("{n}, ");
            print!("{}", &s[n..n+1].red());
        }
        println!(" ");
    }
    
    /* Type text for colored strings */
    /// todo: add option to use different colors
    /// might need dynamic dispatch here
    /// or use a very long match function
    /// I want to be able to take a function as argument
    /// then apply that function to the input s
    /// 
    fn type_text_with_color2(s: String, color: &str)
    {
        
        match color {
            "red" => print!("RED!"),
            _ => print!("No match")
        }
    
        for n in 0..s.len()-1 {
            // print!("{}", &s[n..n+1].red());
            print!("{}", get_red_substring(&s, n));
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        print!("{}", &s[s.len()-1..]);
        println!();
    }
    
    fn type_colored_substring(s: &String, f: fn(s: &String, n: usize) -> ColoredString )
    {
        let sleep_time = std::time::Duration::from_millis(50);
        for n in 0..=s.len()-1 {
            // print!("{}", &s[n..n+1].red());
            print!("{}", f(s,n));
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);

        }
    }
    
    /* get_[colored]_substring() */
    fn get_red_substring(s: &String, n: usize) -> ColoredString
    {
        s[n..n+1].red()
    }
    
    fn get_green_substring(s: &String, n: usize) -> ColoredString
    {

        s[n..n+1].green()
    }
    
    fn get_blue_substring(s: &String, n: usize) -> ColoredString
    {
        s[n..n+1].blue()

    }
}