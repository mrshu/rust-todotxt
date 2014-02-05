

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        finished: bool,
        raw_todo: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}

fn create_task(todo: ~str, id: int) -> Todo {
        let mut task = Todo{id: id, raw_todo: todo.clone(), todo: ~"", finished: false, 
                                priority: '^', contexts: &[], projects: &[]};

        if (todo[0] == "x".as_bytes()[0]) {
                task.finished = true;
        }

        task.todo = todo.clone();

        return task;
}

impl Todo {
        fn to_str(&self) -> ~str {
                return format!("{:d} {:b} ({:c}) '{:s}' \"{:s}\"",
                                self.id, self.finished, self.priority,
                                self.todo, self.raw_todo)
        }
}

fn main () {

        let t = create_task(~"some important task", 1);
        println(t.to_str());
        let x = create_task(~"x some important task", 1);
        println(x.to_str());
}
