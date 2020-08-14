#![allow(dead_code)]
mod odds_and_ends;
mod advanced_topics;
mod lifetime_and_memory;
mod functions;
mod chars_and_strings;
mod std_collections;
mod basics; // looks for "mod.rs" in the basics folder
mod understand_borrow; // from https://doc.rust-lang.org/1.8.0/book
mod traits;
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

fn do_all_borrow(){
    understand_borrow::heap_and_stack::fake_main();
}

fn do_all_traits(){

    traits::basic::basic();
    traits::parameters::parameters();
    traits::into::into();
    traits::drop::trait_drop();
    traits::operator_overloading::operator_overloading();
    traits::static_dispatch::static_dispatch();
    traits::dynamic_dispatch::dynamic_dispatch();
    traits::why_dynamic_dispatch::why();
    traits::vectors_objects_different_types::my_main();

}

fn do_all_lifetime_and_memory(){
    lifetime_and_memory::ownership::lifetime_and_memory();
    lifetime_and_memory::borrowing::borrowing();
    lifetime_and_memory::lifetime::lifetime();
    lifetime_and_memory::lifetime::lifetime_structure_impl();
    lifetime_and_memory::reference_counted_variable::rcv();
    lifetime_and_memory::arc::arc();
    lifetime_and_memory::arc_mutex::arc_mutex();

}

fn do_all_adv_topics(){
    advanced_topics::circular_refs::circular_refs();
    advanced_topics::concurrency::concurrencyv1();
    advanced_topics::concurrency::concurrencyv2();
}

fn do_all_odds_and_ends(){
    odds_and_ends::consuming_crates();
    odds_and_ends::creating_crate();
    odds_and_ends::creating_crate2();
}


fn main(){
    println!("hello main");
    //do_all_basics();
    //do_all_std_collections();
    //do_all_chars_and_strings();
    //do_all_functions();
    //do_all_borrow();
    //do_all_traits();
    //do_all_lifetime_and_memory();
    //do_all_adv_topics();
    do_all_odds_and_ends();
    println!("bye main");
}