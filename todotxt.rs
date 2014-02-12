
pub struct Task {
        id: int,
        todo: ~str,
        priority: char,
        finished: bool,
        raw_todo: ~str,
        create_date: ~str,
        finish_date: ~str,
        contexts: & 'static [~str],
        projects: & 'static [~str]
}

impl Task {
        pub fn to_str(&self) -> ~str {
                return format!("{:d} {:b} ({:c}) |{:s}| '{:s}' \"{:s}\"",
                                self.id, self.finished, self.priority,
                                self.create_date, self.todo, self.raw_todo)
        }

        pub fn create(text: ~str, id: int) -> Task {
                let mut task = Task{id: id, raw_todo: text.clone(), todo: text.clone(),
                                        finished: false, priority: '^',
                                        create_date: ~"", finish_date: ~"",
                                        contexts: &[], projects: &[]};

                let mut todo = text.clone();

                if (todo.slice(0, 2) == "x ") {
                        task.finished = true;
                        todo = todo.slice(2, todo.len()).to_owned();
                }

                let mut chars = todo.chars().to_owned_vec();

                chars = todo.chars().to_owned_vec();
                if (chars[10] == ' ' && chars[4] == '-' &&
                    chars[7] == '-' && task.finished) {
                        task.finish_date = todo.slice(0, 10).to_owned();
                        todo = todo.slice(11, todo.len()).to_owned();
                }

                chars = todo.chars().to_owned_vec();
                if (chars[0] == '(' && chars[2] == ')' &&
                    chars[1].is_uppercase() && chars[3] == ' ') {

                        task.priority = chars[1];
                        todo = todo.slice(4, todo.len()).to_owned();
                }

                chars = todo.chars().to_owned_vec();
                if (chars[10] == ' ' && chars[4] == '-' && chars[7] == '-') {
                        task.create_date = todo.slice(0, 10).to_owned();
                        todo = todo.slice(11, todo.len()).to_owned();
                }

                task.todo = todo;

                return task;
        }
}

pub struct Tasks {
        list: ~[Task]
}

impl Tasks {
        pub fn create() -> Tasks {
                let tasks = Tasks{list: ~[]};

                return tasks;
        }

        pub fn add(&mut self, task: Task) {
                self.list.push(task);
        }

        pub fn add_task_from_string(&mut self, text: ~str) {
                let id = self.len();
                self.list.push(Task::create(text, id));
        }

        pub fn len(&self) -> int {
                return self.list.len() as int;
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

        let ts = Task::create(~"(a) some important task with priority", 1);

        assert_eq!(ts.id , 1);
        assert_eq!(ts.priority , '^');
        assert_eq!(ts.finished , false);
        assert_eq!(ts.todo , ~"(a) some important task with priority");
        assert_eq!(ts.raw_todo , ~"(a) some important task with priority");

}

#[test]
fn simple_todo_with_date_test() {
        let t = Task::create(~"(A) 2014-03-02 some important task with priority", 1);

        assert_eq!(t.id , 1);
        assert_eq!(t.priority , 'A');
        assert_eq!(t.finished , false);
        assert_eq!(t.todo , ~"some important task with priority");
        assert_eq!(t.raw_todo , ~"(A) 2014-03-02 some important task with priority");
        assert_eq!(t.create_date, ~"2014-03-02");

        let ft = Task::create(~"x 2014-03-03 (A) 2014-03-02 some finished important task", 1);

        assert_eq!(ft.id , 1);
        assert_eq!(ft.priority , 'A');
        assert_eq!(ft.finished , true);
        assert_eq!(ft.todo , ~"some finished important task");
        assert_eq!(ft.raw_todo , ~"x 2014-03-03 (A) 2014-03-02 some finished important task");
        assert_eq!(ft.create_date, ~"2014-03-02");
        assert_eq!(ft.finish_date, ~"2014-03-03");

}

#[test]
fn todo_to_str_test() {
        let t = Task::create(~"some important task", 1);

        assert_eq!(t.to_str(), ~"1 false (^) || 'some important task' \"some important task\"")

        let x = Task::create(~"x some important task", 1);
        assert_eq!(x.to_str(), ~"1 true (^) || 'some important task' \"x some important task\"")
}
