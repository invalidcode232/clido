// Todo object for all our todo actions
use std::fs::{File, OpenOptions};
use std::path::Path;

use chrono::Local;
use tabled::Table;

use super::todo::Todo;

pub struct TodoClient<'a> {
    pub file: Option<File>,
    pub path: &'a Path,
}

impl<'a> TodoClient<'a> {
    fn write(&mut self, data: Todo) {
        let mut writer = csv::Writer::from_writer(self.file.as_mut().unwrap());

        let write_res = writer.serialize(data.clone());
        match write_res {
            Ok(_) => println!("todo added: {}", data.todo),
            Err(err) => println!("failed to write {}: {}", self.path.display(), err),
        }
    }

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
    pub fn add(&mut self, todo: &String) {
        if self.file.is_none() {
            return;
        }

        self.write(Todo {
            todo: todo.to_owned(),
            date_added: Local::now().format("%d/%m/%Y %H:%M").to_string(),
        })
    }

    pub fn list(&mut self) {
        if self.file.is_none() {
            return;
        }

        let mut todos: Vec<Todo> = Vec::new();
        let mut reader = csv::Reader::from_reader(self.file.as_mut().unwrap());
        for result in reader.deserialize() {
            let data: Todo = result.unwrap();
            todos.push(data);
        }

        let table_display = Table::new(todos).to_string();
        println!("{}", table_display);
    }
}

// Setting default values
impl<'a> Default for TodoClient<'a> {
    fn default() -> Self {
        TodoClient {
            file: Default::default(),
            path: Path::new("todo.csv"),
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
        todo_client.add(&String::from("test todo"));

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
