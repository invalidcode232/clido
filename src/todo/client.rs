// Todo object for all our todo actions
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use chrono::Local;

use super::todo::Todo;

pub struct TodoClient<'a> {
    pub file: Option<File>,
    pub path: &'a Path,
}

impl<'a> TodoClient<'a> {
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

        let mut writer = csv::Writer::from_path(self.path).unwrap();
        let write_res =
            writer.write_record(&[todo, &Local::now().format("%d/%m/%Y %H:%M").to_string()]);
        match write_res {
            Ok(_) => println!("todo added: {}", todo),
            Err(err) => println!("failed to write {}: {}", self.path.display(), err),
        }

        // match self
        //     .file
        //     .as_mut()
        //     .unwrap()
        //     .write(&[todo.as_bytes(), "\n".as_bytes()].concat())
        // {
        //     Err(err) => panic!("couldn't write {}: {}", self.path.display(), err),
        //     Ok(_) => println!("todo added: {}", todo),
        // };
    }

    pub fn list(&mut self) {
        if self.file.is_none() {
            return;
        }

        let mut todos: Vec<Todo> = Vec::new();
        let mut reader = csv::Reader::from_reader(self.file.as_ref().unwrap());
        for result in reader.records() {
            let data = result.unwrap();
            todos.push(Todo {
                todo: data[0].to_string(),
                date_added: data[1].to_string(),
            })
        }

        // let reader = BufReader::new(self.file.as_mut().unwrap());
        //
        // let mut lines = Vec::new();
        //
        // for line in reader.lines() {
        //     lines.push(line.unwrap());
        // }
        //
        // let lines_iter = lines.into_iter();
        // if lines_iter.clone().count() == 0 {
        //     println!("No todo(s) found, add a todo by using `clido add [todo]`");
        //     return;
        // }
        //
        // println!("Todo(s): ");
        // lines_iter.for_each(|line| {
        //     println!("{}", line);
        // });
    }
}

// Setting default values
impl<'a> Default for TodoClient<'a> {
    fn default() -> Self {
        TodoClient {
            file: Default::default(),
            path: Path::new("todo.txt"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use tempfile::tempdir;

    use super::TodoClient;

    #[test]
    fn test_write_file() {
        // use tempdir for new file creation
        let dir = tempdir().unwrap();
        let path = dir.path().join("todo.txt");
        println!("{:?}", path);

        let mut todo_client = TodoClient {
            path: path.as_path(),
            ..Default::default()
        };

        todo_client.init();
        todo_client.write(&String::from("test todo"));

        // read our file
        // not sure why todo_client.list() doesn't work, however we'll recreate a new BufReader
        // with path_str
        let path_str = path.to_str().unwrap();
        let file = File::open(path_str).unwrap();

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_line(&mut contents).unwrap();

        assert_eq!(contents, "test todo\n")
    }
}
