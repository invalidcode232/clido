pub mod todo;

fn main() {
    let mut todo = todo::Todo {
        ..Default::default()
    };

    todo.init();
    todo.write("The quick brown fox jumps over the lazy dog");
}
