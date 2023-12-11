use std::fmt::Debug;

mod template;

trait MyTrait {}

#[derive(Debug)]
struct MyStruct;

impl MyTrait for MyStruct {}

fn main() {
    print!("====star====");
    let my_struct = MyStruct {};
    test_static(my_struct);
    print!("====end====");
}

fn test_static<T>(t: T)
    where T:'static + Debug{
    print!("data = {:?}", t)
}
