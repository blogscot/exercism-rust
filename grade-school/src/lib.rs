use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade = self.students.entry(grade).or_insert_with(Vec::new);
        grade.push(student.into());
        grade.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students.get(&grade).cloned()
    }
}
