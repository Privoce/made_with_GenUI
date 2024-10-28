use gen_macros::event;
use makepad_widgets::{DefaultNone, ActionDefaultRef};

#[event]
pub enum TestE{
    TestA
}


#[event]
#[derive(Debug, Clone)]
pub enum TestE2{
    TestA2(A2)
}

#[derive(Clone, Debug)]
pub struct A2{
    pub a: f32
}