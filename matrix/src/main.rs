use crate::tests::exercises;
use std::panic::catch_unwind;
mod vector;
pub mod matrix;
pub mod tests;
pub mod linear_algebra;

fn main() {


    let result = catch_unwind(|| {
        exercises::ex00();
        // exercises::ex01();
        // exercises::ex02();
        // exercises::ex03();
        // exercises::ex04();
        // exercises::ex05();
        // exercises::ex06();
        // exercises::ex07();
        // exercises::ex08();
        // exercises::ex09();
        // exercises::ex10();
        // exercises::ex11();
        // exercises::ex12();
        // exercises::ex13();
        // exercises::ex14();
        // exercises::ex15();
    });

    match result {
        Ok(_) => {},
        Err(e) => {println!("Caught a panic : {:?}", e)},
    };
}
