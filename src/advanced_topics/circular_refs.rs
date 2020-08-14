//circular references are complicated in Rust due to the borrow tracking

//student*  <--> course* (many to many)

use std::rc::Rc;
use std::cell::RefCell;

struct Student<'lt>{
    name: String,
    courses: Vec<&'lt Course<'lt>>
}

impl<'lt> Student<'lt> {
    fn new(name: &str) -> Student<'lt>
    {
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct  Course<'lt>{
    name: String,
    students: Vec<&'lt Student<'lt>>
}

impl<'lt> Course<'lt> {
    fn new(name: &str) -> Course<'lt>
    {
        Course{
            name: name.into(),
            students: Vec::new()
        }
    }
    fn add_student(&'lt mut self, student: &'lt Student<'lt>){
        //student.courses.push(self); //error[E0596]: cannot borrow `student.courses` as mutable, as it is behind a `&` reference
        self.students.push(student);
        //RefCell would solve only the borrow problem, not the lifetime one
    }

}


//// VERSION 2 WITH RefCell and RC (ugly version)

struct Student2{
    name: String,
    courses: Vec<Rc<RefCell<Course2>>>
}

impl Student2 {
    fn new(name: &str) -> Student2
    {
        Student2{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct  Course2{
    name: String,
    students: Vec<Rc<RefCell<Student2>>>
}

impl Course2 {
    fn new(name: &str) -> Course2
    {
        Course2{
            name: name.into(),
            students: Vec::new()
        }
    }
    fn add_student2(//cannot be a member function (cannot use self)
        course: Rc<RefCell<Course2>>,
        student: Rc<RefCell<Student2>>)
    {
        //need to use clone because of borrow (clone ref??)
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}


//VERSION 3 REDESIGNED VERSION
// follow the idea of database normalization !
// Vec<student, course, enrollment{course, student}>
struct Student3{
    name: String
}

struct  Course3{
    name: String
}

struct Enrollment3<'lt>{
    student: &'lt Student3,
    course: &'lt Course3
}


impl<'lt> Enrollment3<'lt>{
    fn new(student: &'lt Student3, course:&'lt Course3)
        ->Enrollment3<'lt>
    {
        Enrollment3 {student, course}
    }

}

struct Platform<'lt>{
    enrollments: Vec<Enrollment3<'lt>>
}

impl<'lt> Platform<'lt>{
    fn new() -> Platform<'lt>{
        Platform{
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self,
    student: &'lt Student3,
    course: &'lt  Course3)
    {
        self.enrollments.push(
            Enrollment3::new(student, course)

        )
    }
}
//VERSION 3+ (update student with what they take)
// CONCLUSION: avoid circular references and re-design your application instead !
impl Student3{
    fn courses(&self, platform: Platform) //we should have &Platform
        -> Vec<String>{
        platform.enrollments.iter()
            .filter(|&e| e.student.name ==self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

pub fn circular_refs(){
    //VERSION 1
    let john = Student::new("John");
    let mut course = Course::new("Rust Course");
    course.add_student(&john);//Rc would solve the lifetime problem

    //VERSION 2 (looses a lot of static check that aren't done anymore)
    let john2 = Rc::new(
        RefCell::new(
            Student2::new("John 2")
        )
    );
    let jane = Rc::new(
        RefCell::new(
            Student2::new("Jane")
        )
    );
    let course2 = Course2::new("Rust Course 2");
    let wrap_course = Rc::new(RefCell::new(course2));
    //Course2::add_student2(wrap_course, john2);
    //Course2::add_student2(wrap_course, jane); // error[E0382]: use of moved value: `wrap_course`
    Course2::add_student2(wrap_course.clone(), john2);
    Course2::add_student2(wrap_course, jane); // no error anymore

    //VERSION 3

    //let enrollments = Vec<(Student3, Course3)>::new();
    let john3 = Student3{name: "John3".into()};
    let course3 = Course3{name: "Rust course3 ".into()};
    let course3b = Course3{name: "Dance course3 ".into()};
    let mut p = Platform::new();
    p.enroll(&john3, &course3);
    p.enroll(&john3, &course3b);

    for c in john3.courses(p){
        println!("{} is enrolled in {}", john3.name, c);
    }

}