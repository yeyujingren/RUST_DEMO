// 18.1-1
// if let
// fn main () {
//   let favorite_color: Option<&str> = None;
//   let is_tuesday = false;
//   let age: Result<u8, _> = "34".parse();

//   if let Some(color) = favorite_color {
//     println!("Using your favorite color, {}, as the background", color);
//   } else if is_tuesday {
//     println!("Tuesday is green day");
//   } else if let Ok(age) = age {
//     if age > 30 {
//       println!("Using purple as the background color");
//     } else {
//       println!("Using orangle as the background color");
//     }
//   } else {
//     println!("Using blue as the background color");
//   }
// }

// 18.1-2
// while let
// fn main() {
//   let mut stack = Vec::new();

//   stack.push(1);
//   stack.push(2);
//   stack.push(3);

//   while let Some(top) = stack.pop() {
//     println!("{}", top);
//   }
// }

// 18.1-3
// for 循环
// fn main() {
//   let v = vec!['a', 'b', 'c'];
//   for (index, value) in v.iter().enumerate() {
//     println!("{} is at index {}", value, index);
//   }
// }

// 18。2-1
// // 模式有两种形式： refutable（可反驳的）和irrefutable（不可反驳的）
// fn main() {
//   if let x = 5  {
//     println!("{}", x);
//   }
// }

// 18.3-1
// 所有的模式语法
// fn main() {
//   let x = 1;
//   match x {
//     1 => println!("one"),
//     2 => println!("two"),
//     3 => println!("three"),
//     _ => println!("anything"),
//   }
// }

// // 18。3-2
// fn main() {
//   let x = Some(5);
//   let y = 10;
//   match x {
//     Some(50) => println!("Got 50"),
//     Some(y) => println!("Matched, y = {:?}", y),
//     _ => println!("Default case, x= {:?}", x),
//   }
//   println!("at the end: x = {:?}, y={:?}", x, y);
// }

// 18.3-3 多模式、范围(仅数字和char类型可用)
// fn main() {
//   let x = 3;
//   match x {
//     1 | 2 => println!("one or two"),
//     3..=5 => println!("between three and five"),
//     6 => println!("six"),
//     _ => println!("another")
//   }
// }

// 18.3-4 解构赋值
// struct Point {
//   x: i32,
//   y: i32
// }
//
// fn main() {
//   let p = Point {
//     x: 0,
//     y: 7
//   };
//   let Point {
//     x: a,
//     y: b
//   } = p;
//   assert_eq!(0, a);
//   assert_eq!(7, b);
// }
// 18.3-5 解构潜逃的结构体
// enum Color {
//   Rgb(i32, i32, i32),
//   Hsv(i32, i32, i32)
// }
//
// enum Message {
//   Quit,
//   Move {
//     x: i32,
//     y: i32
//   },
//   Write(String),
//   ChangeColor(Color)
// }
//
// fn main() {
//   let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
//
//   match msg  {
//     Message::ChangeColor(Color::Rgb(r, g, b)) => {
//       println!("Change the color to red {}, green {}, and blue {}", r, g, b);
//     }
//     Message::ChangeColor(Color::Hsv(h, s, v)) => {
//       println!("Change the color to hue {}, saturation {} and value {}", h, s, v);
//     }
//
//     _ => ()
//   }
//   // error demo
//   // let s = Some(String::from("Hello!"));
//   // if let Some(_s) = s {
//   //   println!("found a string");
//   // }
//   //
//   // println!("{:?}", s);
// }

// 18.3-6 ..省略
// fn main() {
//   let numbers = (2, 4, 6, 8, 16, 32);
//   match numbers {
//     (fir, .., second, last) => {
//       println!("Some numbers: {}, {}, {}", fir, second, last);
//     }
//   }
// }

// 18.3-7 匹配守卫： 一个指定于match分支模式之后的额外的if条件，他必须也满足才能选择此分支
// fn main() {
//   let num = Some(4);
//   match num {
//     Some(x) if x > 5 => println!("bigger than five: {}", x),
//     Some(x) => println!("{}", x),
//     None => ()
//   }
// }

// 18.3-8 @绑定
// 允许我们在创建一个存放值的同时，测试其值是否匹配模式
enum Message {
  Hello {
    id: i32
  }
}

fn main() {
  let msg = Message::Hello {
    id: 5
  };

  match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
      println!("Found an id in range: {}", id_variable);
    },
    Message::Hello { id: 10..=12 } => {
      println!("FOund an id in another range");
    },
    Message::Hello { id } => {
      println!("Found some other id: {}", id);
    }
  }
}
