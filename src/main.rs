// use std::thread;
// use std::time::Duration;
// fn main() {
//     let v = vec![1,2,3];
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });
//     // let handle = thread::spawn(|| {
//     //     for i in 1..10 {
//     //         println!("hi number {} from the spawned thread!", i);
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });
//     // for i in 1..5 {
//     //     println!("hi, number {} from the main thread", i);
//     //     thread::sleep(Duration::from_millis(1));
//     // }
//     // drop(v);
//     handle.join().unwrap();
// }

// 16.2-1
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         // println!("val is {}", val);
//     });

//     let received = rx.recv().unwrap();
//     println!("Got, {}", received);
// }


// 16.2-2
// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread")
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("message"),
//             String::from("for"),
//             String::from("U")
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got, {}",received);
//     }
// }


// 16.3-1 互斥器
// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m = {:?}", m);
// }

// 16.3-2
// use std::sync::{Mutex, Arc};
// use std::thread;
// // use std::rc::Rc;

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter); // 增加一个索引
//         let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//            *num += 1;
//         });
//        handles.push(handle)
//     }


//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("result: {}", *counter.lock().unwrap());
// }

// 16.4-1 使用sync和Send trait的可扩展并发

fn main() {
    
}
