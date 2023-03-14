/** TEST  **/
/// `$ cargo test`
#[cfg(test)]
mod tests {
    use crate::scroll;
    use crate::print_colored_substring;
    use crate::get_red_substring;

    #[test] // <-- annotation that indicates test runner that this function is a test
    fn test_scroll2(){
        scroll("This is run in test", 40);
    }

    #[test]
    fn test_takes_color_fn(){

    }

    #[test]
    fn test_print_colored_substring(){
        print_colored_substring(&"hello".to_string(), get_red_substring);
    }

    pub fn test_scroll() {
        let title = "--- Turtles All the Way Down. ---";
        scroll_colored("--- Turtles All The Way Down. ---\n\n".red(), 40);
        analyze_colored_string("--- Turtles All The Way Down. ---\n\n".red());
        print_colored_char(String::from(title), "red");
    
        scroll("Did you know our universe lies on the back of a turtle?\n",20);
    
        scroll("And what, my friend does that turtle stand upon?\n", 20);
    
        scroll("Clever question, my friend. It stands upon another turtle.\n", 20);
        // ERROR not found in this scope
        /*
        155 |     crate::turtle_down::turtle_down::turtles_all_the_way_down(10);
        |                         ^^^^^^^^^^^ private module
         */
        crate::turtle_down::turtle_down_public::turtles_all_the_way_down(10);
    }
    
    pub fn test_read_file(){
        let path_to_file = "test.txt";
        // check if file exists first
        
        let contents = read_file(path_to_file);
        print_file_contents(&contents);
        print_colored_substring(&"hello".to_string(), get_red_substring);
        print_colored_substring(&contents, get_blue_substring);
        // print_colored_char(String::from(contents), "red");
    }