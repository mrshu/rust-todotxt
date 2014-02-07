
pub struct Task {
        id: int,
        todo: ~str,
        priority: char,
        finished: bool,
        raw_todo: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}


impl Task {
        pub fn to_str(&self) -> ~str {
                return format!("{:d} {:b} ({:c}) '{:s}' \"{:s}\"",
                                self.id, self.finished, self.priority,
                                self.todo, self.raw_todo)
        }

        pub fn create(todo: ~str, id: int) -> Task {
                let mut task = Task{id: id, raw_todo: todo.clone(), todo: todo.clone(),
                                        finished: false, priority: '^',
                                        contexts: &[], projects: &[]};

                let mut t = todo.clone();

                if (t.slice(0, 2) == "x ") {
                        task.finished = true;
                        t = t.slice(2, t.len()).to_owned();
                }

                //if (todo[0])
                task.todo = t;

                return task;
        }
}

#[test]
fn simple_todo_create_test() {
        let t = Task::create(~"some important task", 1);

        assert_eq!(t.id , 1);
        assert_eq!(t.priority , '^');
        assert_eq!(t.finished , false);
        assert_eq!(t.todo , ~"some important task");
        assert_eq!(t.raw_todo , ~"some important task");
}

#[test]
fn simple_todo_with_priority_test() {
        let t = Task::create(~"(A) some important task with priority", 1);

        assert_eq!(t.id , 1);
        assert_eq!(t.priority , '^');
        assert_eq!(t.finished , false);
        assert_eq!(t.todo , ~"some important task with priority");
        assert_eq!(t.raw_todo , ~"(A) some important task with priority");
}

#[test]
fn todo_to_str_test() {
        let t = Task::create(~"some important task", 1);

        assert_eq!(t.to_str(), ~"1 false (^) 'some important task' \"some important task\"")

        let x = Task::create(~"x some important task", 1);
        assert_eq!(x.to_str(), ~"1 true (^) 'some important task' \"x some important task\"")
}
