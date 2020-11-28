// student* --- *course

use std::rc::Rc;
use std::cell::RefCell;


struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student
{
    fn new(name: &str) -> Student
    {
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course
{
    fn new(name: &str) -> Course
    {
        Course{
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

pub fn main() {
    let john = Rc::new(RefCell::new(Student::new("John")));
    let jane = Rc::new(RefCell::new(Student::new("Jane")));


    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));

    // course.add_student(john); // Rc

    Course::add_student(magic_course.clone(), john);
    Course::add_student(magic_course, jane);

}
