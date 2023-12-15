use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
struct Sun;

trait Super {}


fn main() {
    let sun = Sun;
    let zero = call_on_ref_zero(sun);
}

impl PartialEq<i32> for &Sun {
    fn eq(&self, other: &i32) -> bool {
        todo!()
    }
}

fn call_on_ref_zero<F>(f: F) -> bool
    where for <'a>
        &'a F: PartialEq<&'a i32>
{
    let zero = 0;
    &f == &zero
}

