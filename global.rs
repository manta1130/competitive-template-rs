pub use std::cell::RefCell;
pub use std::rc::Rc;
use std::thread::LocalKey;

#[macro_export]
#[allow(unused_macros)]
macro_rules! global_init {
    ($x:ident : $t:ty,$i:expr) => {
        thread_local! {
            pub static $x:Rc<RefCell<$t>>=Rc::new(RefCell::new($i));
        }
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! glm {
    ($x:ident) => {
        global_access(&($x)).borrow_mut()
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! gl {
    ($x:ident) => {
        global_access(&($x)).borrow()
    };
}

pub fn global_access<T>(p: &'static LocalKey<Rc<RefCell<T>>>) -> Rc<RefCell<T>> {
    p.with(|q| q.clone())
}
