// #[macro_use]
// Using this allows you to have static that required code to be executed at runtime in order to be initalized
// extern crate lazy_static;

use std::{thread, time};

use std::fs;

// use std::path::Path;

pub struct StringBuilder {

}

impl StringBuilder {
    pub fn concat(str1: &str, str2: &str) -> String {
        format!("{}{}", str1, str2)
    }
}


pub struct TypeWriter {

}


impl TypeWriter {
    pub fn clear_console() {
        print!("{}[2J", 27 as char)
    }
    pub fn clear_console2(){
        print!("{}[2J", 33 as char)
    }
}

/* send a control character to clear terminal screen*/
pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

/// see: https://stackoverflow.com/questions/2388090/how-to-delete-and-replace-last-line-in-the-terminal-using-bash
pub fn clear_line() {
    print!("{}[K", 33 as char);
}

/// see: https://stackoverflow.com/questions/28823788/how-do-i-clear-the-current-line-of-stdout
pub fn clear_line2() {
    print!("\r");
}

/** File Functions **/
fn read_file(path_to_file: &str) -> String{
    let contents = fs::read_to_string(path_to_file)
        .expect("Should be able to read the file."); // -> Result<String>
    contents
}
/// Print file content
fn print_file_contents(input: &String){
    println!("File contents: {input}");
}

/// Sleep for milliseconds
pub fn sleep_ms(ms_to_wait: u64)
{
    let time_to_wait = time::Duration::from_millis(ms_to_wait);
    thread::sleep(time_to_wait);
}

/// Takes two &str(string slices)/string literals and concatenates both then returns combined string
pub fn concat(str1: &str, str2: &str) -> String {
    format!("{}{}", str1, str2)
}

pub mod term {
    use crate::concat;

    use std::io::{self, Write};

    use colored::Colorize;
    use colored::ColoredString;
    use colored::Color;

    
    /* lazy_static!
    {
        static ref PAUSE_TIME: u64 = 20;
    } */

    /** Type out text**/
    // Should probably combine these functions by letting it take a trait object
    /// Print out text with typewriter effect
    pub fn typewriter(s: &str) 
    {
        let ms = 50;
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    
    /// Print out text with typewriter effect in X ms
    pub fn typewriterms(s: &str, ms: u64) 
    {
        
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }

    /// This doesn't work at all
    /// Extracting the characters of the ColoredString means it loses
    /// it's color information
    #[deprecated]
    pub fn type_color(s: ColoredString)
    {
        const PAUSE_TIME_MS: u64 = 20;
        let sleep_time = std::time::Duration::from_millis(PAUSE_TIME_MS);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Couldn't flush the terminal buffer!");
            std::thread::sleep(sleep_time);
        }
    }

    /// This doesn't work at all
    /// Extracting the characters of the ColoredString means it loses
    /// it's color information
    #[deprecated]
    fn type_text_colored_in_ms(s: ColoredString, ms: u64) 
    {
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    

    /// Get information from input string
    pub fn analyze_colored_string(s: ColoredString)
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
    
    /// Print out text with typewriter effect with a color
    /// e.g.
    /// ```
    /// type_text_colored("red", "i'm red!!!");
    /// ```
    pub fn typewritec(color: &str, s: &str)
    {
        let s2 = concat(s, " ");

        match color {
            "red" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_red_char);
            },
            "oj" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_oj_char);      
            },
            "yellow" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_yellow_char);              
            },
            "blue" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_blue_char);
            },
            "green" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_green_char);
            },
            _ => print!("No match")
        }
    
        

        print!("{}", &s2[s2.len()-1..]);
        println!();
    }
    
    /// Takes callback function that prints desired color
    /// e.g. type_colored_char(|s2| get_red_char);
    fn type_colored_char(string: &String, 
        f: impl Fn(&String, usize) -> ColoredString)
    {
        let sleep_time = std::time::Duration::from_millis(50);
        
        for i in 0..=string.len()-1 {
            // print!("{}", &s[n..n+1].red());
            print!("{}", f(string,i));
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }

    }

    fn type_rotating_slash(){
        let buffer="\\|/-";
    }

    /* get_[colored]_substring() */
    fn get_red_char(s: &String, i: usize) -> ColoredString{
        s[i..i+1].red()
    }
    
    fn get_oj_char(s: &String, i: usize) -> ColoredString {
        s[i..i+1].truecolor(255,165,0)
    }

    fn get_yellow_char(s: &String, i: usize) -> ColoredString {
        s[i..i+1].yellow()
    }

    fn get_green_char(s: &String, i: usize) -> ColoredString{
        s[i..i+1].green()
    }
    
    fn get_blue_char(s: &String, i: usize) -> ColoredString{
        s[i..i+1].blue()
    }

    
    fn get_char_colored(s: &String, n: usize, color: &str ){
        match color {
            "red" => {
                println!("matched {}", color);
            },
            "green" => {
                println!("matched {}", color);
            },
            "blue" => {
                println!("matched {}", color);
            },
            _ => println!("No matching colors"),
        }
    }
}