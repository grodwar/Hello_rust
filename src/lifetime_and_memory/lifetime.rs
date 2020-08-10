//part 1
struct Person{
    name: String
}

struct Company<'my_life_time> //'z is the company struct instance lifetime
{
    name:String,
    //ceo: &Person //error[E0106]: missing lifetime specifier
    ceo: &'my_life_time Person //error[E0106]: missing lifetime specifier

}
//part 2
impl Person {
    // we get lifetime elision below, which actually is like the line  below
    fn get_ref_name(&self) -> &String
    //fn get_ref_name<'a>(&'a self) -> &'a String
    //warning: Explicit lifetimes given in parameter types where they could be elided
    {
        &self.name// it loos like it gives a ref to a String that may not exist anymore
    }

}

pub fn lifetime()
{
    //part 1
    // 'static is  a definition of a variable lifetime
    // in this case it means it will live until the end of the program
    //&'static str  toto;

    let boss = Person{name:String::from("Elon musk")};
    let _tesla = Company{name:String::from("Tesla"), ceo:&boss };

    //if we do these 2 lines, we get:
    //error[E0505]: cannot move out of `boss` because it is borrowed
    //drop(boss);
    //println!("{}",_tesla.name);
    //part 2 (counter example)
    let  z: &String;
    {//artificial scope
        let p  = Person{name:String::from("John Doe")};
        z = p.get_ref_name();
        println!("inside z:{:?}", z)
    }
    //println!("outside z:{:?}", z) //error[E0597]: `p` does not live long enough

}
//// //// //// //// //// //// //// //// //// //// //// //// //// //// //// ////
//// LIFETIME IN STRUCTURE IMPLEMENTATION
struct Person2<'lt>{//lt ==> lifetime
    //name: & str //error[E0106]: missing lifetime specifier
    name: &'lt str

}

impl<'lt> Person2<'lt> {
    //fn talk(&self) {//error[E0726]: implicit elided lifetime not allowed here
    fn talk(&self) {
        println!("Hi! My name is {}", self.name);
    }

    
}



pub fn lifetime_structure_impl()
{
    let person = Person2{name:"Dmitri"};
    person.talk();

}