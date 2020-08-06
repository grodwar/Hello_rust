
//one of feature of Rust is to use pre-defined traits, like 'into'
// it allows automatic conversion where possible

struct Person{
    name: String
}

impl Person
{
    //Version 1
   /* fn new(name: &str) -> Person
    {
        Person {name: name.to_string()}
    }*/

    //Version 2
    //this means S convertible into String
    /*fn new<S: Into<String>>(name:S) -> Person
    {
        Person {name: name.into()}
    }*/
    //Version 3
    //same as version 2 but with a where clause
    fn new<S>(name:S)  -> Person where  S:Into<String>
    {
        Person {name: name.into()}
    }
}
pub fn into(){

    let _john = Person::new("John ");// its all ok

    let name = "jane".to_string();
    //Version 1
    //let _jane = Person::new(name.as_ref());//this tedious !
    // we want Person to initialize itself not only from &str but also from String !
    //Version 2
    let _jane = Person::new(name);
    //Version 3
    let name3 = "jane".to_string();
    let _jane = Person::new(name3);


}