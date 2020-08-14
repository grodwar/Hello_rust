#[cfg(test)]

mod tests{
    extern crate phrases2;

    #[test]
    fn english_greeting_correct(){
        assert_eq!("hello 2", phrases2::greetings2::english2::hello());
    }
    // then just run "cargo test" in the module directory for the current example

    #[test]
    fn english_goodbye_correct(){
        assert_eq!("goodbye 2",  phrases2::greetings2::english2::goodbye());
    }

    #[test]
    #[should_panic]
    fn english_goodbye_correct2(){
        assert_eq!("au revoir 2", phrases2::greetings2::english2::goodbye());
    }

    #[test]
    #[ignore]
    fn english_bad_test(){
        assert_eq!("patatra", phrases2::greetings2::english2::goodbye());
    }
}