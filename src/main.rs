use std::any::Any;
use std::fmt::Debug;
use std::mem::needs_drop;


trait MyTrait: Any + AsSuper {}

#[derive(Debug)]
struct MyStruct;

impl MyTrait for MyStruct {}

impl<T: MyTrait> AsSuper for T {
    fn as_super(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

trait AsSuper {
    fn as_super(self: Box<Self>) -> Box<dyn Any>;
}

fn main() {
    //given
    let vec: Vec<Box<dyn MyTrait>> = vec![Box::new(MyStruct), Box::new(MyStruct)];
    let value_vec: Vec<Box<MyStruct>> = vec.into_iter()
        .filter_map(|obj| {
            Some(obj.as_super().downcast::<MyStruct>().unwrap())
        })
        .collect();

    // 打印转换后的向量  
    println!("{:?}", value_vec);
}

fn test_static<T>(t: T) where T: 'static + Debug {
    print!("data = {:?}", t)
}
