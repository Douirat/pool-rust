#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Role {
        match s {
            _ if s.eq_ignore_ascii_case("ceo") => Role::CEO,
            _ if s.eq_ignore_ascii_case("manager") => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self{
            grade:None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
       let r = role.into();
       let mut w = Box::new(Worker{
            role: r,
            name: name.to_string(),
            next: None,
       });

        w.next = self.grade.take();
        self.grade = Some(w);
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(node) = self.grade.take() {
            let Worker { name, next, .. } = *node;
            self.grade = next;
            Some(name)
        } else {
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, Role)> {
      if let Some(x) = &self.grade{
        return Some((x.name.clone(), x.role.clone()));
      } else{
          None
      }
    }
    }


