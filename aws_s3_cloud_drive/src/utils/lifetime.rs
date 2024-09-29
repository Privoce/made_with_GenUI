#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lifetime {
    Init,
    InProcess,
    Destroy,
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Init
    }
}

impl Lifetime {
    pub fn next(&self) -> Self {
        match self {
            Lifetime::Init => Lifetime::InProcess,
            Lifetime::InProcess => Lifetime::Destroy,
            Lifetime::Destroy => Lifetime::Init,
        }
    }
    pub fn init(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Init,
        }
    }
    pub fn in_process(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::InProcess,
        }
    }
    pub fn destroy(&self) -> LifetimeExecutor {
        LifetimeExecutor {
            current: *self,
            target: Lifetime::Destroy,
        }
    }
}

pub trait Executor {
    type Item;
    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce();
}

pub struct LifetimeExecutor {
    current: Lifetime,
    target: Lifetime,
}
impl Executor for LifetimeExecutor {
    type Item = Lifetime;

    fn execute<F>(&self, f: F) -> Option<Self::Item>
    where
        F: FnOnce(),
    {
        if self.current == self.target {
            f();
            //back next
            Some(self.current.next())
        } else {
            None
        }
    }
}
