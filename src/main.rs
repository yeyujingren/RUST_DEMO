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
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("U")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got, {}",received);
    }
}
