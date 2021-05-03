// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students: Vec<Student>,
}

#[derive(Clone)]
struct Student {
    name: String,
    grade: u32,
}

impl School {
    pub fn new() -> School {
        School { students: vec![] }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push(Student {
            grade,
            name: student.to_string(),
        })
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut students = self
            .students
            .iter()
            .cloned()
            .map(|s| s.grade)
            .collect::<Vec<u32>>();
        students.sort();
        students.dedup();
        students
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self
            .students
            .iter()
            .cloned()
            .filter(|s| s.grade == grade)
            .map(|s| s.name)
            .collect::<Vec<String>>();
        students.sort();
        students
    }
}
