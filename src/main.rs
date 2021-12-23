// use minigrep::{Draw, Screen, Button};

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // do something
//     }
// }

// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("maybe"),
//                     String::from("No")
//                 ]
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("Ok")
//             })
//         ]
//     };

//     screen.run();

//     // let err_screen_demo = Screen {
//     //     components: vec![
//     //         Box::new(String::from("Hi"))
//     //     ]
//     // };
// }


// // 17.3-1 状态模式
// use minigrep::Post;
// fn main() {
//   let mut post = Post::new();
//   post.add_text("I ate a salad for lunch today");
//   assert_eq!("", post.content());
//   post.request_review();
//   assert_eq!("", post.content());

//   post.approve();
//   assert_eq!("I ate a salad for lunch today", post.content());
// }

// 17.3-2 状态模式优化
use minigrep::Post;
fn main() {
  let mut post = Post::new();
  post.add_text("I ate a salad for lunch today");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("I ate a salad for lunch today", post.content())
}
