
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

        pub fn create(text: ~str, id: int) -> Task {
                let mut task = Task{id: id, raw_todo: text.clone(), todo: text.clone(),
                                        finished: false, priority: '^',
                                        contexts: &[], projects: &[]};

                let mut todo = text.clone();

                if (todo.slice(0, 2) == "x ") {
                        task.finished = true;
                        todo = todo.slice(2, todo.len()).to_owned();
                }

                let chars = todo.chars().to_owned_vec();
                if (chars[0] == '(' && chars[2] == ')' &&
                    chars[1].is_uppercase() && chars[3] == ' ') {

                        task.priority = chars[1];
                        todo = todo.slice(4, todo.len()).to_owned();
                }

                task.todo = todo;

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
        assert_eq!(t.priority , 'A');
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
