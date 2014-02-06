
pub struct Todo {
        id: int,
        todo: ~str,
        priority: char,
        finished: bool,
        raw_todo: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}



impl Todo {
        pub fn to_str(&self) -> ~str {
                return format!("{:d} {:b} ({:c}) '{:s}' \"{:s}\"",
                                self.id, self.finished, self.priority,
                                self.todo, self.raw_todo)
        }

        pub fn create(todo: ~str, id: int) -> Todo {
                let mut task = Todo{id: id, raw_todo: todo.clone(), todo: todo.clone(),
                                        finished: false, priority: '^',
                                        contexts: &[], projects: &[]};

                if (todo.slice(0, 2) == "x ") {
                        task.finished = true;
                        task.todo = todo.slice(2, todo.len()).to_owned()
                }

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

#[test]
fn todo_to_str_test() {
        let t = Todo::create(~"some important task", 1);

        assert_eq!(t.to_str(), ~"1 false (^) 'some important task' \"some important task\"")

        let x = Todo::create(~"x some important task", 1);
        assert_eq!(x.to_str(), ~"1 true (^) 'some important task' \"x some important task\"")
}
