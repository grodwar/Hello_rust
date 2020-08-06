#![allow(unused_variables)]
#![allow(dead_code)]
// drop is (quite) equivalent to a destructor in other programming language
// yet it is quite rarely useful, beside to when you want to know when a var is deleted


struct Creature{
    name: String
}

//factory
impl Creature {
    fn new(name: &str) -> Creature
    {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature
{
    fn drop(&mut self)
    {
        println!("{} leaves the game", self.name);
        //apparently the default drop is done
    }
}

pub fn trait_drop(){
    let x = 3;
    let goblin = Creature::new("Bob");
    println!("The game proceeds");
    //goblin.drop();// error: explicit use of destructor method
    drop(goblin);
    println!("The game proceeds without Bob");
    //let goblin2 =  goblin;// use of moved value: `goblin

    //trying to trick the compiler
    let _clever: Creature;
    {
        println!("Beginning of scope");
        let goblin2 = Creature::new("Jeff");
        _clever = goblin2;
        println!("end of scope");
    }

}