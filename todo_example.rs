
mod todotxt;

fn main () {

        let mut tasks = ~[];

        let t = ::todotxt::Task::create(~"some important task", 1);
        println(t.to_str());

        tasks.push(t);
        let x = ::todotxt::Task::create(~"x some important task", 1);
        println(x.to_str());

        tasks.push(x);
}
