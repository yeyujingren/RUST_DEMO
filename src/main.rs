// use crate::List::{ Cons, Nil };
// use std::ops::Deref;
// use std::mem::drop;
// use std::rc::Rc;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil
// }

// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// struct CustomSmartPointer {
//     data: String
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data: {}", self.data);
//     }
// }

// fn main() {
    // let list = Cons(2, Cons(3, Nil));
    // let list = Cons(1,
    //     Box::new(
    //         Cons(2,
    //             Box::new(
    //                 Cons(3,
    //                     Box::new(Nil)
    //                 )
    //             )
    //         )
    //     )
    // );

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let m = MyBox::new(String::from("Rust"));
    // let string = String::from("123");
    // hello(&(*string)[..]);

    // let c = CustomSmartPointer {
    //     data: String::from("my stiff")
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff")
    // };
    // println!("CustomSmartPointer created.");
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main");

    // let a = Rc::new(Cons(5,
    //     Rc::new(
    //         Cons(10,
    //             Rc::new(Nil)
    //         )
    //     )
    // ));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, a.clone());
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after goes out of scope = {}", Rc::strong_count(&a));
// }

// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }


// 15.6-1 循环引用
// use std::rc::Rc;
// use std::cell::RefCell;
// use crate::List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None
//         }
//     }
// }



// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after change a = {}", Rc::strong_count(&a));
// }

// 15.6-2 Weak引用
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
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
            children: RefCell::new(vec![Rc::clone(&leaf)])
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
