
trait Printable
{
    fn format(&self) -> String;
}
impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32:  {}",*self)
    }
}
impl Printable for String
{
    fn format(&self) -> String
    {
        format!("String:  {}",*self)
    }
}

// version 2
fn print_it(z:&dyn Printable) //"dyn" was not present in the lecture (its a new feature?)
{
    println!("{}", z.format());
}// dynanmic dispatch
// at runtime this method has to determine the actual type of z
// to use the proper method the format of i32 or the one of String

pub fn dynamic_dispatch()
{
    let a = 23;
    let b = "hello".to_string();
    let _c = 42.42;
    print_it(&a);// print_it doesn't know the type of a... just that &a is a pointer
    // to a printable
    print_it(&b);
    //print_it(&_c);// at compile time: error[E0277]: the trait bound
    // `{float}: traits::dynamic_dispatch::Printable` is not satisfied

}