use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

// The `std ops::Add` trait is used to specifiy the functionality of `+` 
// Here we make sure Add<Bar> - the trait for addtion with a RHS of type `Bar`.

// Foo + Bar = FooBar 

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> FooBar {
        FooBar
    }
}



fn main() {
    println!("Hello, world!");


}