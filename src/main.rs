// 19.1-1 解引用裸指针
// fn main() {
//   let mut num = 5;
//   let r1 = &num as *const i32;
//   let r2 = &mut num as *mut i32;

//   let address = 0x012345usize;
//   let r = address as *const i32;

//   unsafe {
//     println!("r1 is: {}", *r1);
//     println!("r2 is: {}", *r2);
//   }
// }

// 19.1-2 调用不安全函数或方法
// use std::slice;
// unsafe fn dangerous() {}

// fn main () {
//   // dangerous();
//   unsafe {
//     dangerous();
//   }
// 
//   let mut v = vec![1,2,3,4,5,6];
//   let r = &mut v[..];
//   let (a, b) = r.split_at_mut(3);
//   assert_eq!(a, &mut [1,2,3]);
//   assert_eq!(b, &mut [4,5,6]);
// }

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//   let len = slice.len();
//   let ptr = slice.as_mut_ptr();

//   assert!(mid <= len);
//   unsafe {
//     (
//       slice::from_raw_parts_mut(ptr, mid),
//       slice::from_raw_parts_mut(ptr.add(mid), len-mid)
//     )
//   }
// }

// 19.1-2_2 使用extern函数调用外部代码
// demo 引入C中abs函数
// extern "C" {
//   fn abs(input: i32) -> i32;
// }

// fn main() {
//   unsafe {
//     println!("Absolute value of -3 according to C: {}", abs(-3))
//   }
// }

// 19.1-2_3 使用 extern 创建一个允许其他语言调用的rust函数接口
// #[no_mangle] // 禁止使用rust编译器的name mangling是必要的，因为不通编译器会执行不通的方式mangle函数名
// pub extern "C" fn call_from_c() {
//   println!("Just called a Rust function from C!");
// }


// 19.1-3 访问或修改可变静态变量
// 使用static来标识，与常量之间的区别：
// a、静态变量中的值有一个固定的内存地址；常量则允许在任何被用的时候复制其数据
// b、静态变量是可变的，因此，访问和修改静态变量都是不安全的。
// static mut COUNTER: u32 = 0;
// fn add_to_count(inc: u32) {
//   unsafe {
//     COUNTER += inc;
//   }
// }

// fn main() {
//   add_to_count(3);

//   unsafe {
//     println!("COUNTER is: {}", COUNTER);
//   }
// }


// 19.1-4 实现不安全 trait
// unsafe trait Foo {
//   // something
// }
// unsafe impl Foo for i32 {
//   // todo
// }

// 访问 union 的字段
// 主要用于和C代码中的联合交互。

// 19.2 高级 trait

// 19.2-1 关联类型在 trait 定义中指定占位符号类型
// 使用关联类型，而不是范型的原因
// a、范型 可以针对一个 trait 实现多个同名的 struct （多态），
// 这样则在每次使用的时候，都必须指明类型来表明希望使用trait的哪一个实现
// b、关联类型： 只能实现实现一个同名的 struct，因此不需要在每次使用时都标注
// pub trait Iterator {
//   type Item;
//   fn next(&mut self) -> Option<Self::Item>;
// }

// 19.2-2 默认范型参数和运算符重载
// 19.2-2_1 point相加
// use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//   x: i32,
//   y: i32
// }

// impl Add for Point {
//   type Output = Point;

//   fn add(self, other: Point) -> Point {
//     Point {
//       x: self.x + other.x,
//       y: self.y + other.y
//     }
//   }
// }

// fn main() {
//   assert_eq!(
//     Point {
//       x: 1, y: 0
//     } + Point {
//       x: 2, y: 3
//     },
//     Point {
//       x: 3, y: 3
//     }
//   )
// }

// 19.2-2_2 millimeters + meters
// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//   type Output = Millimeters;

//   fn add(self, other: Meters) -> Millimeters {
//     Millimeters(self.0 + (other.0 * 1000))
//   }
// }

// 19.2-3 完全限定语法与消除歧义：调用相同名称的方法
// 19.2-3_1 消除歧义
// trait Pilot {
//   fn fly(&self);
// }

// trait Wizard {
//   fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//   fn fly(&self) {
//     println!("This is your captain speaking.");
//   }
// }

// impl Wizard for Human {
//   fn fly(&self) {
//     println!("Up!");
//   }
// }

// impl Human {
//   fn fly(&self) {
//     println!("waving arms furiously");
//   }
// }

// fn main() {
//   let persion = Human;
//   Pilot::fly(&persion);
//   Wizard::fly(&persion);
//   persion.fly();
// }

// 19.2-3_2 完全限定
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
// trait Animal {
//   fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//   fn baby_name() -> String {
//     String::from("Spot")
//   }
// }

// impl Animal for Dog {
//   fn baby_name() -> String {
//     String::from("puppy")
//   }
// }

// // fn main() {
// //   println!("A baby dog is called a {}", <Dog as Animal>::baby_name())
// // }

// // 19.2-4 父 trait 用于在另一个 trait 中使用某 trait 的功能
// use std::fmt;
// trait OutlinePrint: fmt::Display {
//   fn outline_print(&self) {
//     let output = self.to_string();
//     let len = output.len();
//     println!("{}", "*".repeat(len + 4));
//     println!("*{}*", " ".repeat(len + 2));
//     println!("* {} *", output);
//     println!("*{}*", " ".repeat(len + 2));
//     println!("{}", "*".repeat(len + 4));
//   }
// }

// struct Point {
//   x: i32,
//   y: i32
// }

// impl fmt::Display for Point {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "({}, {}", self.x, self.y)
//   }
// }

// impl OutlinePrint for Point {}

// fn main() {
//   let point = Point {
//     x: 2,
//     y: 3
//   };

//   point.outline_print();
// }

// 19.2-5 newtype 模式用以在外部类型上实现外部 trait
// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "[{}]", self.0.join(", "))
//   }
// }

// fn main() {
//   let w = Wrapper(vec![String::from("Hello"), String::from("world")]);

//   println!("w = {}", w);
// }

// 19.3 高级类型
// 19.3-1 类型别名用来创建类型同义词：将复杂冗长的类型申明简洁化
// type Kilometers = i32;

// fn main () {
//   let x: i32 = 5;
//   let y: Kilometers = 5;
//   println!("x+y={}", x+y)

// }

// 19.3-2 never

// WARNING! dont use this func in main;
// fn bor() -> ! {
//   loop {

//   }
// }

// 19.4 高级函数与闭包
// 19.4-1 函数指针

fn add_one(x:i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn main() {
  let answer = do_twice(add_one, 5);
  println!("The answer is {}", answer);
}

// 19.4-2 返回闭包
fn returns_closeure_error_demo () -> Fn(i32) -> i32 {
  |x| x+1
}

fn returns_closeure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}
