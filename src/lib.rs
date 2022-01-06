// 19.5 宏
// 19.5-1 简化版 vec!

// #[macro_export]
// macro_rules! vec {
//   ( $( $x:expr ),* ) => {
//     {
//       let mut temp_vec = Vec::new();
//       $(
//         temp_vec.push($x)
//       )*
//       temp_vec
//     }
//   };
// }

// 19.5-2 过程宏（procedural macros）
// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

// 19.5-2_1  过程宏
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // 将 rust 代码解析为语法树，以便后续操作
  let ast = syn::parse(input).unwrap();

  // 构建 trait 实现
  impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl HelloMacro for #name {
      fn hello_macro() {
        println!("Hello, Macro! My name is {}", stringify(#name));
      }
    }
  };
  gen.into()
}