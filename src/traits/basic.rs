trait Animal{

    //part 1
    fn name(&self) -> &'static str;
    fn talk(&self)
    {
        println!("{} cannot talk", self.name()); // a default implementation
    }
    //part 2
    // create function
    // static function (cause it doesn't take self as a parameter)
    // that returns the type of the implementor "Self"
    fn create(name: &'static str) -> Self;
}

struct Human{
    name : &'static str
}

impl Animal for Human
{
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says hello!",self.name())
    }
    //part2
    fn create(name: &'static str) -> Human //it returns Human not Self !!!
    {
        Human{name: name}//remember its a return statement !
    }
}


struct Cat{
    name : &'static str
}
impl Animal for Cat
{
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says Meow!",self.name())
    }
    //part2
    fn create(name: &'static str) -> Cat //it returns Human not Self !!!
    {
        Cat{name: name}
    }

}

//part 4

trait Summable<T>{// T-->  generic type
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self
        {
            result+= *x;//dont forget to dereference x!!!
        }
        result
    }
}

pub fn basic(){
    //part 1
    let h = Human{name:"John"};
    println!("Here is: {}", h.name());
    h.talk();

    let c = Cat{name:"Felix the cat"};
    println!("Here is: {}", c.name());
    c.talk();

    //part 2
    let h2 = Human::create("Bob");
    println!("Here is: {}", h2.name());
    h2.talk();

    let c2 = Cat::create("Misty the pussy");
    println!("Here is: {}", c2.name());
    c2.talk();

    //part 3
    //let h3 = Animal::create("Bob"); // type annotations needed
    let h3:Human = Animal::create("Marcel");
    println!("Here is: {}", h3.name());
    h3.talk();

    //parrt 4

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}