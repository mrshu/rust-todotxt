

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        raw_todo: ~str
}

fn create_task(todo: ~str, id: int) -> Todo {
        let task = Todo{id: id, raw_todo: todo, todo: ~"", priority: '^'};

        return task;
}

fn main () {

}
