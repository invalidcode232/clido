use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Todo<'a> {
    pub file: Option<File>,
    pub path_name: &'a str,
}

// TODO: Seperate each function with its own file
impl<'a> Todo<'a> {
    pub fn init(&mut self) {
        let path = Path::new(self.path_name);
        let path_name = path.display();

        self.file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)
        {
            Err(err) => panic!("couldn't create {}: {}", path_name, err),
            Ok(file) => Some(file),
        };
    }

    pub fn write(&mut self, todo: &str) {
        if self.file.is_none() {
            return;
        }

        match self.file.as_mut().unwrap().write(todo.as_bytes()) {
            Err(err) => panic!("couldn't write {}: {}", self.path_name, err),
            Ok(_) => println!("{} written successfully", self.path_name),
        };
    }
}

impl<'a> Default for Todo<'a> {
    // TODO: try initializing the file in the default function (without the need for Todo.init()
    // function)

    fn default() -> Self {
        Todo {
            file: Default::default(),
            path_name: "todo.txt",
        }
    }
}
