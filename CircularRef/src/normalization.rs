// students
// courses
// Vec<Enrollment{course, student}>

struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(
            Enrollment::new(student, course)
        )
    }
}

pub fn main() {
    let john = Student {name: "John".into()};
    let course = Course {name: "Intro to Rust".into()};
    let course2 = Course {name: "Rust Programming".into()};

    let mut p = Platform::new();
    p.enroll(&john, &course);
    p.enroll(&john, &course2);

    for c in john.courses(p) {
        println!("John is taking {}", c);
    }
}
