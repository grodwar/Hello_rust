// similar to Rc but multi-threaded!
//use std::rc::Rc; // not necessary anymore as we use Arc now
use std::thread;
//use std::rc::Arc;
use std::sync::Arc;

struct Person{
    //name:String
    //name:Rc<String>
    name:Arc<String>
}
impl Person{
    //fn new(name: Rc<String>)-> Person
    fn new(name: Arc<String>)-> Person
    {
        Person{name:name}
    }

    fn greet(&self)
    {
        println!("Hi! My name is {}", self.name);
    }
}
pub fn arc(){
    //let name = Rc::new("john".to_string());
    let name = Arc::new("Arc john".to_string());
    let person = Person::new(name.clone());
    let t = thread::spawn(move || {
            person.greet()
        }); // error[E0277]: `std::rc::Rc<std::string::String>` cannot be sent between threads safely
    println!("name = {}", name);
    t.join().unwrap();
}