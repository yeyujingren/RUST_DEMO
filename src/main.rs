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
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

fn main() {
  add_to_count(3);

  unsafe {
    println!("COUNTER is: {}", COUNTER);
  }
}


// 19.1-4 实现不安全 trait
unsafe trait Foo {
  // something
}
unsafe impl Foo for i32 {
  // todo
}

// 访问 union 的字段
// 主要用于和C代码中的联合交互。

