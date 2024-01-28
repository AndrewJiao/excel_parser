fn main() {
    do_echo(A)
}

trait MyTrait {
    fn echo(&self) {
        println!("this is trait")
    }
}

struct A;

impl MyTrait for A {
    fn echo(&self) {
        (*self).echo()
    }
}

fn do_echo<T: MyTrait>(t:T) {
    t.echo()
}