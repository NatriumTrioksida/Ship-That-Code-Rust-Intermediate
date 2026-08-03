use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;

    println!("{}", counter.borrow());
}
