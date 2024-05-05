// // // // // // // // pub fn add(left: usize, right: usize) -> usize {
// // // // // // // //     left + right
// // // // // // // // }

// // // // // // // // #[cfg(test)]
// // // // // // // // mod tests {
// // // // // // // //     use super::*;

// // // // // // // //     #[test]
// // // // // // // //     fn exploration() {
// // // // // // // //         assert_eq!(2+2, 4);
// // // // // // // //     }

// // // // // // // //     #[test]
// // // // // // // //     fn another() {
// // // // // // // //         panic!("Make this test fail");
// // // // // // // //     }
// // // // // // // // }


// // // // // // // #[derive(Debug)]
// // // // // // // struct Rectangle {
// // // // // // //     width: u32,
// // // // // // //     height: u32,
// // // // // // // }

// // // // // // // impl Rectangle {
// // // // // // //     fn can_hold(&self, other: &Rectangle) -> bool {
// // // // // // //         self.width < other.width && self.height > other.height
// // // // // // //     }
// // // // // // // }

// // // // // // // #[cfg(test)]
// // // // // // // mod test1 {
// // // // // // //     use super::*;

// // // // // // //     #[test]
// // // // // // //     fn larger_can_hold_smaller() {
// // // // // // //         let larger = Rectangle {
// // // // // // //             width: 8,
// // // // // // //             height: 7,
// // // // // // //         };
// // // // // // //         let smaller = Rectangle {
// // // // // // //             width: 5,
// // // // // // //             height: 1,
// // // // // // //         };

// // // // // // //         assert!(larger.can_hold(&smaller));
// // // // // // //     }

// // // // // // //     #[test]
// // // // // // //     fn smaller_can_hold_larger() {
// // // // // // //         let larger = Rectangle {
// // // // // // //             width: 8,
// // // // // // //             height: 7,
// // // // // // //         };
// // // // // // //         let smaller = Rectangle {
// // // // // // //             width: 5,
// // // // // // //             height: 1,
// // // // // // //         };

// // // // // // //         assert!(!smaller.can_hold(&larger));
// // // // // // //     }
// // // // // // // }


// // // // // // // pub fn add_two(a: i32) -> i32 {
// // // // // // //     a + 3
// // // // // // // }

// // // // // // // #[cfg(test)]
// // // // // // // mod test2 {
// // // // // // //     use super::*;

// // // // // // //     #[test]
// // // // // // //     fn it_adds_two() {
// // // // // // //         assert_eq!(4, add_two(2));
// // // // // // //     }
// // // // // // // }


// // // // // // pub fn greeting(name: &str) -> String {
// // // // // //     String::from("Hello!")
// // // // // // }


// // // // // // #[cfg(test)]
// // // // // // mod test3 {
// // // // // //     use super::*;

// // // // // //     #[test]
// // // // // //     fn greeting_contains_name() {
// // // // // //         let result = greeting("Carol");
// // // // // //         assert!(result.contains("Carol"));
// // // // // //     }
// // // // // // }

// // // // // pub struct Guess {
// // // // //     value: i32,
// // // // // }

// // // // // impl Guess {
// // // // //     pub fn new(value: i32) -> Guess {
// // // // //         if value < 1 || value > 100 {
// // // // //             panic!("Guess value must be between 1 and 100, got {}.", value);
// // // // //         }

// // // // //         Guess { value }
// // // // //     }
// // // // // }

// // // // // #[cfg(test)]
// // // // // mod test4 {
// // // // //     use super::*;

// // // // //     #[test]
// // // // //     #[should_panic]
// // // // //     fn greater_than_100() {
// // // // //         Guess::new(200);
// // // // //     }
// // // // // }

// // // // // --snip--

// // // // pub struct Guess {
// // // //     value: i32,
// // // // }

// // // // impl Guess {
// // // //     pub fn new(value: i32) -> Guess {
// // // //         if value < 1 {
// // // //             panic!(
// // // //                 "Guess value must be greater than or equal to 1, got {}.",
// // // //                 value
// // // //             );
// // // //         } else if value > 100 {
// // // //             panic!(
// // // //                 "Guess value must be less than or equal to 100, got {}.",
// // // //                 value
// // // //             );
// // // //         }

// // // //         Guess { value }
// // // //     }
// // // // }

// // // // #[cfg(test)]
// // // // mod test5 {
// // // //     use super::*;

// // // //     #[test]
// // // //     #[should_panic(expected = "less than or equal to 100")]
// // // //     fn greater_than_100() {
// // // //         Guess::new(200);
// // // //     }
// // // // }

// // // #[cfg(test)]
// // // mod test6 {
// // //     #[test]
// // //     fn it_works() -> Result<(), String> {
// // //         if 2 + 2 == 4 {
// // //             Ok(())
// // //         } else {
// // //             Err(String::from("two plus two does not equal four"))
// // //         }
// // //     }
// // // }

// // fn prints_and_returns_10(a: i32) -> i32 {
// //     println!("I got the value {}", a);
// //     10
// // }

// // #[cfg(test)]
// // mod test7 {
// //     use super::*;

// //     #[test]
// //     fn this_test_will_pass() {
// //         let value = prints_and_returns_10(4);
// //         assert_eq!(4, value);
// //     }

// //     #[test]
// //     fn this_test_will_fail() {
// //         let value = prints_and_returns_10(8);
// //         assert_eq!(10, value);
// //     }
// // }

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test8 {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

