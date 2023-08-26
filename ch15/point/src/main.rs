use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use run::run3;

#[derive(Debug, PartialEq)]
enum List_v1 {
    Cons(i32, Rc<List_v1>),
    Nil,
}

impl List_v1 {
    fn value(&self) -> Option<i32> {
        match self {
            List_v1::Cons(num, next) => Some(*num),
            List_v1::Nil => None,
        }
    }
    fn next(&self) -> &List_v1 {
        match self {
            List_v1::Cons(num, next) => next,
            List_v1::Nil => &List_v1::Nil,
        }
    }
}

#[derive(Debug, PartialEq)]
enum List_v2 {
    Cons(Rc<RefCell<i32>>, Rc<List_v2>),
    Nil,
}

#[derive(Debug, PartialEq)]
enum List_v3 {
    Cons(i32, RefCell<Rc<List_v3>>),
    Nil,
}

impl List_v3 {
    fn tail(&self) -> Option<&RefCell<Rc<List_v3>>> {
        match self {
            Self::Cons(_, item) => Some(item),
            Self::Nil => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

mod run {
    use super::*;

    pub fn run0() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        // let d = CustomSmartPointer {
        //     data: String::from("other stuff"),
        // };
        println!("CustomSmartPointers created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
    pub fn run1() {
        use crate::List_v1::{Cons, Nil};

        let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
        let mut x = &list;
        while *x != Nil {
            let num = x.value().unwrap();
            println!("{}", num);
            x = x.next();
        }
    }
    pub fn run2() {
        use crate::List_v1::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    pub fn run3() {
        use crate::List_v2::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("v after = {:?}", c);
    }
    pub fn run4() {
        use List_v3::{Cons, Nil};
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&a));
        println!("a rc count after changing a = {}", Rc::strong_count(&b));
    }
    pub fn run5() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    pub fn run6() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch)
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf)
            );
        }
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
}

fn main() {
    run::run6();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_my_box() {
        let s = String::from("Rust");
        let mut m = MyBox::new(s.clone());
        assert_eq!(&s, m.deref().deref());
    }
}
