use std::thread;
use std::time::Duration;

struct Cacher<T> 
    where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>
    }

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
    {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
        calculation,
        value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if v != arg {
                    let cur_v = (self.calculation)(arg);
                    self.value = Some(cur_v);
                    return cur_v;
                } else {
                    v
                }
            },
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {

    // 初代目
    // let expensive_closure = |num: u32| -> u32 {
    //   println!("calculation slowly...");
    //   thread::sleep(Duration::from_secs(2));
    //   num
    // };
    
    // 一代目
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_result
            // expensive_closure(intensity)
            expensive_closure.value(intensity)
        );
        println!(
            "next, do {} situps",
            // expensive_result
            // expensive_closure(intensity)
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                // expensive_result
                // expensive_closure(intensity)
                expensive_closure.value(intensity)
            );
        }
    }
}

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 我们不生产迭代器，我们只是迭代器的化妆师
pub fn interator_dector() {
    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    for item in v2 {
        println!("I GOT: {}", item);
    }
}


#[test]
fn test_cacher_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// 迭代器获取所有权
#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    // println!("GOT: {:?}", v1);
}

// 谨慎使用for、sum，每次使用，都有一个迭代器将要被消费！😂
#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    let s = String::from("1245");
}

// 在闭包中使用环境变量
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")}
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot")}
        ]
    )
}

// 实现Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly () {
    let counter = Counter::new();

    for item in counter {
        println!("{}", item)
    }
}


#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("{}", sum);
}