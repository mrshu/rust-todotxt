
mod todotxt;

fn main () {

        let t = ::todotxt::Todo::create(~"some important task", 1);
        println(t.to_str());
        let x = ::todotxt::Todo::create(~"x some important task", 1);
        println(x.to_str());
}
