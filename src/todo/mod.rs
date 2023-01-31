// Todo object for all our todo actions
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct Todo<'a> {
    pub file: Option<File>,
    pub path: &'a Path,
}

impl<'a> Todo<'a> {
    // Create a todo file if not exists, as well as storing the File to our struct
    pub fn init(&mut self) {
        self.file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(self.path)
        {
            Err(err) => panic!("couldn't create {}: {}", self.path.display(), err),
            Ok(file) => Some(file),
        };
    }

    // Write a new todo to our file
    pub fn write(&mut self, todo: &String) {
        if self.file.is_none() {
            return;
        }

        match self
            .file
            .as_mut()
            .unwrap()
            .write(&[todo.as_bytes(), "\n".as_bytes()].concat())
        {
            Err(err) => panic!("couldn't write {}: {}", self.path.display(), err),
            Ok(_) => println!("todo added: {}", todo),
        };
    }

    pub fn list(&mut self) {
        if self.file.is_none() {
            return;
        }

        println!("Todo(s): ");
        let reader = BufReader::new(self.file.as_mut().unwrap());
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}

// Setting default values
impl<'a> Default for Todo<'a> {
    fn default() -> Self {
        Todo {
            file: Default::default(),
            path: Path::new("todo.txt"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::Todo;

    #[test]
    fn test_write_file() -> Result<(), String> {
        let test_dir = temp_dir().join("\\todo.txt");
        let mut todo_client = Todo {
            path: test_dir.as_path(),
            ..Default::default()
        };

        todo_client.write(&String::from("lol"));

        Ok(())
    }
}
