/*

    Rust's closures are anon function you save in a var, or pass as arguments to other fns
    You can create closures in one place and then call the closure to eval in a diff context
    Unlike functions, closures can capture values from the scope

    Closures don't require you to annotate the types of the parameters or the return value like fn function do
    Type annotations are required because they expose an interface
    Closures are short and relevant only in a narrow context
*/

/* Example closure v fn */
// fn add_one_v1 (x: u32) -> u32  { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x| { x + 1 };
// let add_one_v4 = |x| x + 1;

use std::thread;
use std::time::Duration;

/* holds a closure and optional result value */
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }

    // #[test]
    // fn call_with_different_values() {
    //     let mut c = Cacher::new(|a| a);

    //     let v1 = c.value(1);
    //     let v2 = c.value(2);

    //     assert_eq!(v2, 2);
    // }
}


fn simualted_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Notice how we declare it as a let
// let expensive_closure = |num: u32| -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     num
// };

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity > 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    )
}