#![allow(dead_code)]

mod functions;
mod chars_and_strings;
mod std_collections;
mod basics; // looks for "mod.rs" in the basics folder
/* //used for when those files where in the same directory
mod generics;
mod pattern_matching;
mod tuples;
mod slices;
mod arrays;
mod option_t;
mod combination_lock;
mod match_statement;
mod loops;
mod if_statement;
mod types_and_variables;
mod stack_and_heap;
mod struct_enum_unions;
 */

fn do_all_basics(){
    basics::types_and_variables::types_and_variables();
    basics::stack_and_heap::stack_and_heap();
    basics::if_statement::if_statement();
    basics::loops::while_loop();
    basics::loops::for_loop();
    basics::match_statement::match_statement();
    basics::combination_lock::combination_lock();
    basics::struct_enum_unions::structures();
    basics::struct_enum_unions::enumerations();
    basics::struct_enum_unions::unions();
    basics::option_t::option_t();
    basics::arrays::arrays();
    basics::slices::slices();
    basics::tuples::tuples();
    basics::pattern_matching::pattern_matching();
    basics::generics::generics();
}

fn do_all_std_collections()
{
    std_collections::vectors();
    std_collections::hashmaps();
    std_collections::hashmaps_v2();
    std_collections::sets();
}
fn do_all_chars_and_strings()
{
    chars_and_strings::strings();
    chars_and_strings::formatting_strings();
    chars_and_strings::number_guessing_game();
}

fn do_all_functions(){
    //functions::function_and_arguments();
    //functions::methods();
    //functions::closures();
    functions::higher_order_functions();
}

fn main(){
    println!("hello main");
    //do_all_basics();
    //do_all_std_collections();
    //do_all_chars_and_strings();
    do_all_functions();
    println!("bye main");
}