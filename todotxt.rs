

struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        finished: bool,
        raw_todo: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}



impl Todo {
        fn to_str(&self) -> ~str {
                return format!("{:d} {:b} ({:c}) '{:s}' \"{:s}\"",
                                self.id, self.finished, self.priority,
                                self.todo, self.raw_todo)
        }

        fn create(todo: ~str, id: int) -> Todo {
                let mut task = Todo{id: id, raw_todo: todo.clone(), todo: ~"",
                                        finished: false, priority: '^',
                                        contexts: &[], projects: &[]};

                if (todo[0] == "x".as_bytes()[0]) {
                        task.finished = true;
                }

                task.todo = todo.clone();

                return task;
        }
}

#[test]
fn simple_todo_create_test() {
        let t = Todo::create(~"some important task", 1);

        assert!(t.id == 1);
        assert!(t.priority == '^');
        assert!(t.finished == false);
        assert!(t.todo == ~"some important task");
        assert!(t.raw_todo == ~"some important task");
}

fn main () {

        let t = Todo::create(~"some important task", 1);
        println(t.to_str());
        let x = Todo::create(~"x some important task", 1);
        println(x.to_str());
}
