// 17.1
// pub struct AveragedCollection {
//   list: Vec<i32>,
//   average: f64
// }

// impl AveragedCollection {
//   pub fn add(&mut self, value: i32) {
//     self.list.push(value);
//     self.update_average();
//   }

//   pub fn remove(&mut self) -> Option<i32> {
//     let result = self.list.pop();
//     match result {
//       Some(val) => {
//         self.update_average();
//         Some(val)
//       },
//       None => None
//     }
//   }

//   pub fn average(&self) -> f64 {
//     self.average
//   }

//   fn update_average(&mut self) {
//     let total: i32 = self.list.iter().sum();
//     self.average = total as f64 / self.list.len() as f64;
//   }
// }


// 17.2
// pub trait Draw {
//   fn draw(&self);
// }

// pub struct Screen {
//   pub components: Vec<Box<dyn Draw>>
// }

// impl Screen {
//   pub fn run(&self) {
//     for component in self.components.iter() {
//       component.draw();
//     }
//   }
// }

// // pub struct Screen<T: Draw> {
// //   pub components: Vec<T>
// // }

// // impl<T> Screen<T>
// //   where T: Draw {
// //     pub fn run (&self) {
// //       for component in self.components.iter() {
// //         component.draw()
// //       }
// //     }
// //   }

// pub struct Button {
//   pub width: u32,
//   pub height: u32,
//   pub label: String
// }

// impl Draw for Button {
//   fn draw(&self) {

//   }
// }

// 17.3-1
// pub struct Post {
//   state: Option<Box<dyn State>>,
//   content: String
// }

// // 定义blob实体
// impl Post {
//   pub fn new() -> Post {
//     Post {
//       state: Some(Box::new(Draft {})),
//       content: String::new(),
//     }
//   }
//   pub fn add_text(&mut self, text: &str) {
//     self.content.push_str(text);
//   }
//   pub fn content(&self) -> &str {
//     self.state.as_ref().unwrap().content(self)
//   }
//   pub fn request_review(&mut self) {
//     if let Some(s) = self.state.take() {
//       self.state = Some(s.request_review())
//     }
//   }
//   pub fn approve(&mut self) {
//     if let Some(s) = self.state.take() {
//       self.state = Some(s.approve());
//     }
//   }
// }

// trait State {
//   fn request_review(self: Box<Self>) -> Box<dyn State>;
//   fn approve(self: Box<Self>) -> Box<dyn State>;
//   fn content<'a>(&self, post: &'a Post) -> &'a str {
//     ""
//   }
// }

// // 草稿箱
// struct Draft {}
// impl State for Draft {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     Box::new(PendingReview {})
//   }
//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }
// }

// // 审核阶段
// struct PendingReview {}
// impl State for PendingReview {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     Box::new(Publish {})
//   }
// }

// // 发布阶段
// struct Publish {}
// impl State for Publish {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn content<'a>(&self, post: &'a Post) -> &'a str {
//     &post.content
//   }
// }


// 17.3-2
pub struct Post {
  content: String
}

pub struct DraftPost {
  content: String
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content
    }
  }
}

pub struct PendingReviewPost {
  content: String
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content
    }
  }
}
