

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        raw_todo: ~str
}

fn create_task(todo: ~str, id: int) -> Todo {
        let task = Todo{id: id, raw_todo: todo, todo: ~"", priority: '^'};

        task.todo = todo;

        return task;
}

fn main () {

        let t = create_task(~"some important task", 1);
}
