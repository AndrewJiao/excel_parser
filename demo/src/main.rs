fn main() {
    let mut super_a: Box<dyn Super> = generate().as_dyn_super();
    {
        super_a = (generate()).as_dyn_super();
    }
    super_a.do_super();
}

trait Super: AsDynSuper {
    fn do_super(&self) {
        println!("super");
    }
}

trait AsDynSuper {
    fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
        where Self: 'a;
}

// 为所有 T: Super + Sized 实现 AsDynSuper  特征的方法as_dyn_super
impl<T: Super + Sized> AsDynSuper for T {
    fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
        where Self: 'a,
    {
        self
    }
}

trait Sub: Super {}

fn upcast(obj: Box<dyn Sub>) -> Box<dyn Super> {
    obj.as_dyn_super()
}

fn generate() -> Box<dyn Sub> {
    Box::new(A)
}

struct A;

impl Super for A {}

impl Sub for A {}