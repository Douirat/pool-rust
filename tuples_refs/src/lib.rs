pub struct Student(
    pub id: u32,
    pub first_name: &str,
    pub last_name: &str,
)



pub fn id(student: &Student) -> u32 {
    student.id
}

pub fn first_name(student: &Student) -> &str {
student.first_name
}

pub fn last_name(student: &Student) -> &str {
    student.last_name
}