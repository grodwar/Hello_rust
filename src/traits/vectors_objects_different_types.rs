//from basic.rs
trait Animal{
    // static: called as Animal::create()
    // returns the type of the implementor
    // fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self)
    {
        println!("{} cannot talk", self.name()); // a default implementation
    }

}

struct Human{
    name : &'static str
}

impl Animal for Human
{
    //fn create(name: &'static str) -> Human //it returns Human not Self !!!
    //{Human{name: name}}
    fn name(&self) -> &'static str
    {self.name}

    fn talk(&self)
    {println!("Hello my name is {}",self.name())}
}


struct Cat{
    name : &'static str
}
impl Animal for Cat
{
    //fn create(name: &'static str) -> Cat
    //{Cat{name: name}}
    fn name(&self) -> &'static str
    {self.name}
    fn talk(&self)
    {println!("{} says Meow!",self.name())}
}

//solution version 1
// but it leads to annoying details in the usage
enum CreatureEnum
{
    Human(Human),
    Cat(Cat)
}
pub fn my_main()
{

    let mut creatures = Vec::new();
    //by default this doesn't work cause creatures will be expected humans as it is the first one
    // to be pushed in
    // so we get "error[E0308]: mismatched types"
    //creatures.push(Human{name:"John"});
    //creatures.push(Cat{name:"Misty"});

    //solution version 1
    // not creatures is a vector of the enum "Creature" (Vec<Creature>)
    creatures.push(CreatureEnum::Human(
        Human{name:"John"}
    ));
    creatures.push(CreatureEnum::Cat(
        Cat{name:"Misty"}
    ));

    for c in creatures
    {
        //c.talk();// error[E0599]: no method named `talk` found for enum
        // `traits::vectors_objects_different_types::CreatureEnum` in the current scope
        // solution version 1
        match c{
            CreatureEnum::Human(h) => h.talk(),
            CreatureEnum::Cat(c) => c.talk(),

        }
    }

    // bad idea version 2
    //error[E0277]: the size for values of type
    // `dyn traits::vectors_objects_different_types::Animal` cannot be known at compilation time
    //let mut animals:Vec<Animal> = Vec::new();
    //animals.push(Human{name:"John"});
    //animals.push(Cat{name:"Misty"});

    //solution version 3
    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human{name:"Marcel"}));
    animals.push(Box::new(Cat{name:"Kitty"}));

    for a in animals.iter()
    {
        a.talk();
    }


}