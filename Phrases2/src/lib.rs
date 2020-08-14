// this is to show HOW TO CREATE YOUR OWN CRATE
pub mod greetings2
{
    pub mod english2;
    pub mod french2
    {
        pub fn hello() -> String {"bonjour 2".to_string()}
        pub fn goodbye() -> String {"au revoir 2".to_string()}
    }
    //now update/create the Cargo.toml file for this greetings module
    // cargo build in the Phrases directory
    // got some warnings that the functions are never used
    // back to Cargo.toml of the project
}
//// TESTING PART BASIC VERSION
// see the tests directory
/*
#[test]
fn english_greeting_correct(){
    assert_eq!("hello 2", greetings2::english2::hello());
}
// then just run "cargo test" in the module directory for the current example

#[test]
fn english_goodbye_correct(){
    assert_eq!("goodbye 2", greetings2::english2::goodbye());
}

#[test]
#[should_panic]
fn english_goodbye_correct2(){
    assert_eq!("au revoir 2", greetings2::english2::goodbye());
}

#[test]
#[ignore]
fn english_bad_test(){
    assert_eq!("patatra", greetings2::english2::goodbye());
}

 */