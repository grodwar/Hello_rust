//there is an alternative to ownership and borrowing
//using a reference counting
// not that it is used for single thread !
use std::rc::Rc;

struct Person{
    //name:String
    name:Rc<String>
}
impl Person{
    //fn new(name: String)-> Person
    fn new(name: Rc<String>)-> Person
    {
        Person{name:name}
    }

    fn greet(&self)
    {
         println!("Hi! My name is {}", self.name);
    }
}
pub fn rcv(){
    let name = Rc::new("john".to_string());
    let person = Person::new(name.clone());
    person.greet();
    println!("his name is {}", name);//error[E0382]: borrow of moved value: `name`x` (BEFORE USING RC)

    //part 2, counting the current references
    let name2 = Rc::new("Bob".to_string());
    println!("name2 is {} and has {} strong pointers", name2, Rc::strong_count(&name2));
    {
        let person2 = Person::new(name2.clone());
        println!("(inside scope) name2 is {} and has {} strong pointers", name2, Rc::strong_count(&name2));
        person2.greet();
    }
    println!("(after scope) name2 is {} and has {} strong pointers", name2, Rc::strong_count(&name2));

}