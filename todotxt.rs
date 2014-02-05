

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        raw_todo: ~str
}

fn create_task(todo: ~str, id: int) -> Todo {
        let mut task = Todo{id: id, raw_todo: todo.clone(), todo: ~"", priority: '^'};

        task.todo = todo.clone();

        return task;
}

fn main () {

        let t = create_task(~"some important task", 1);
}
