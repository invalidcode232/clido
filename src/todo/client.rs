// Todo object for all our todo actions
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use chrono::Local;
use tabled::{Style, Table};
extern crate xdg;

use super::todo::Todo;

pub struct TodoClient {
    pub file: Option<File>,
    pub path: PathBuf,
}

impl TodoClient {
    // Write a new record to csv
    fn write(&mut self, data: Todo) {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(self.file.as_mut().unwrap());

        let write_res = writer.serialize(data.clone());
        match write_res {
            Ok(_) => println!("todo added: {}", data.todo),
            Err(err) => println!("failed to write {}: {}", self.path.display(), err),
        }
    }

    fn write_all(&mut self, data: Vec<Todo>) {
        let mut file_write_all = match OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(self.path.as_path())
        {
            Ok(file) => Some(file),
            Err(err) => panic!("error opening file: {}", err),
        };

        let mut writer = csv::WriterBuilder::new().from_writer(file_write_all.as_mut().unwrap());

        // data.iter().for_each(|todo| {
        //     let write_res = writer.serialize(todo.clone());
        //     match write_res {
        //         Ok(_) => Default::default(),
        //         Err(err) => println!("failed to write {}: {}", self.path.display(), err),
        //     }
        // })

        for (i, todo) in data.iter().enumerate() {
            let mut todo = todo.clone();
            todo.index = i as i32;

            let write_res = writer.serialize(todo.clone());
            match write_res {
                Ok(_) => Default::default(),
                Err(err) => println!("failed to write {}: {}", self.path.display(), err),
            }
        }
    }

    // Reads all records in the csv, returns a vector of Todo
    fn read(&mut self) -> Vec<Todo> {
        let mut todos: Vec<Todo> = Vec::new();
        let mut reader = csv::Reader::from_reader(self.file.as_mut().unwrap());
        for result in reader.deserialize() {
            let data: Todo = result.unwrap();
            todos.push(data);
        }

        todos
    }

    // Create a todo file if not exists, as well as storing the File to our struct
    pub fn init(&mut self) {
        self.file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(self.path.as_path())
        {
            Err(err) => panic!("couldn't create {}: {}", self.path.display(), err),
            Ok(file) => Some(file),
        };

        let file = self.file.as_mut().unwrap();
        if file.metadata().unwrap().len() == 0 {
            let mut writer = csv::Writer::from_writer(file);
            let write_res = writer.write_record(&["index", "todo", "date_added", "done"]);
            match write_res {
                Ok(_) => println!("wrote headers to todo.csv"),
                Err(_) => panic!("failed to write headers"),
            }
        }
    }

    // Write a new todo to our file
    pub fn add(&mut self, todo: &String) {
        if self.file.is_none() {
            return;
        }

        let todo_count = self.read().into_iter().count();
        let new_todo_index = todo_count as i32;
        self.write(Todo {
            todo: todo.to_owned(),
            date_added: Local::now().format("%d/%m/%Y %H:%M").to_string(),
            done: false,
            index: new_todo_index,
        })
    }

    // Lists all todos in a table format
    pub fn list(&mut self) {
        if self.file.is_none() {
            return;
        }

        let todos = self.read();
        // Convert to a Table for display
        let todo_table = Table::new(todos).with(Style::rounded()).to_string();
        println!("{}", todo_table);
    }

    // Set todo done
    pub fn set_done(&mut self, index: i32, is_done: bool) {
        let mut todos = self.read();
        match todos.get_mut(index as usize) {
            Some(todo) => {
                todo.done = is_done;
                println!("successfully set index {} to {}", index, is_done);
            }
            None => {
                println!("index not found");
                return;
            }
        };

        self.write_all(todos);
    }

    // Remove todo
    pub fn remove_todo(&mut self, index: i32) {
        let mut todos = self.read();

        // Check if todo exists, then remove it if it does
        match todos.get(index as usize) {
            Some(_) => {
                todos.remove(index as usize);
                println!("successfully deleted index {}", index);
            }
            None => {
                println!("index not found");
                return;
            }
        };

        self.write_all(todos);
    }
}

// Setting default values
impl Default for TodoClient {
    fn default() -> Self {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("clido").unwrap();
        let csv_path = xdg_dirs
            .place_data_file("todo.csv")
            .expect("could not get todo directory");

        TodoClient {
            file: Default::default(),
            path: csv_path.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::TodoClient;

    #[test]
    fn test_write_file() {
        // use tempdir for new file creation
        let dir = tempdir().unwrap();
        let path = dir.path().join("todo.txt");
        println!("{:?}", path);

        let mut todo_client = TodoClient {
            path,
            ..Default::default()
        };

        todo_client.init();
        todo_client.add(&String::from("test todo"));
    }
}
