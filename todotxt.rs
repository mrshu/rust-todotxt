

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        raw_todo: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}

fn create_task(todo: ~str, id: int) -> Todo {
        let mut task = Todo{id: id, raw_todo: todo.clone(), todo: ~"", priority: '^',
                                contexts: &[], projects: &[]};

        if (todo[0] == "x".as_bytes()[0]) {
                println("finished");
        }
        task.todo = todo.clone();

        return task;
}

fn main () {

        let t = create_task(~"some important task", 1);
        let x = create_task(~"x some important task", 1);
}
