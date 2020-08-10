// the previous version did not mutate anything so it was thread safe
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

struct Person{
    name:Arc<String>,
    //state:Arc<String>
    state:Arc<Mutex<String>>
}
impl Person{
    //fn new(name: Rc<String>)-> Person
    //fn new(name: Arc<String>, state: ArcString>)-> Person
    fn new(name: Arc<String>, state: Arc<Mutex<String>>)-> Person
    {
        Person{name:name, state:state}
    }

    fn greet(&self)
    {
        //println!("Hi! My name is {}", self.name);
        //self.state.clear(); // error[E0596]: cannot borrow data in an `Arc` as mutable
        // self.state.push_str("excited"); // error[E0596]: cannot borrow data in an `Arc` as mutable
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi! My name is {} and my state is {}", self.name, state.as_str());
    }
}
pub fn arc_mutex(){
    //let name = Rc::new("john".to_string());
    let name = Arc::new("Arc john".to_string());
    //let state = Arc::new("bored".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
            person.greet()
        }); // error[E0277]: `std::rc::Rc<std::string::String>` cannot be sent between threads safely
    //println!("name = {}, state = {}", name, state);
    println!("name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}