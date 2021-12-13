pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  message: &'a T,
  value: usize,
  max: usize
}

impl<'a, T> LimitTracker<'a, T> 
  where T: Messenger {
    pub fn new(message: & T, max: usize) -> LimitTracker<T> {
      LimitTracker {
        message,
        value: 0,
        max
      }
    }

    pub fn set_value(&mut self, value: usize) {
      self.value = value;
      let percentage_of_max = self.value as f64 / self.max as f64;
      if percentage_of_max >= 1.0 {
        self.message.send("Error: you are over your quota!");
      } else if percentage_of_max >= 0.9 {
        self.message.send("Urgent warning: You've used up over 90% your quota!");
      } else if percentage_of_max >= 0.75 {
        self.message.send("Warning: You've used up over 75% your quota!");
      }
    }
  }

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessage {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessage {
    fn new () -> MockMessage {
      MockMessage {
        sent_messages: RefCell::new(vec![])
      }
    }
  }

  impl Messenger for MockMessage {
    fn send(&self, message: &str) {
      // self.send_message.push(String::from(message));
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn in_send_an_over_75_percent_warning_message() {
    let mock_message = MockMessage::new();

    let mut limit_tracker = LimitTracker::new(&mock_message, 100);
    limit_tracker.set_value(80);

    assert_eq!(mock_message.sent_messages.borrow().len(), 1);
  }

  // Err demo
  // impl Messenger for MockMessage {
  //   fn send(&self, message: &str) {
  //     let mut one_borrow = self.sent_messages.borrow_mut();
  //     let mut two_borrow = self.sent_messages.borrow_mut();

  //     one_borrow.push(String::from(message));
  //     two_borrow.push(String::from(message));
  //   }
  // }
}

// ERR TEST
// #[cfg(test)]
// mod tests {
//   use super::*;
//   struct MockMessage {
//     send_message: Vec<String>
//   }

//   impl MockMessage {
//     fn new() -> MockMessage {
//       MockMessage {
//         send_message: vec![]
//       }
//     }
//   }

//   impl Messenger for MockMessage {
//     fn send(&self, message: &str) {
//       self.send_message.push(String::from(message));
//     }
//   }

//   #[test]
//   fn test_demo() {
//     let mock_messager = MockMessage::new();
//     let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
    
//     limit_tracker.set_value(80);

//     assert_eq!(mock_messager.send_message.len(), 1);

//   }
// }
