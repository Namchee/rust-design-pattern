// Prototype is a design pattern that allows you to copy existing
// object without being too dependant of the source class
//
// The main problem of naive copying:
// 1. We don't know the value of private methods
// 2. Cannot handle interface (trait) well
// 3. Coupling of class
//
// In Rust, we have the `Clone` trait, so we don't have to implement
// it from scratch. This is very convenient due to Rust ownership model.
//
// FAQ
//
// 1. Copy vs Clone?
// - Copy is implicit and inexpensive bit-wise copy while Clone performs full object duplication
//   Rust doesn't allow re-implementation of copy. If you want to copy a struct with props
//   that doesn't allow Copy, you have to Clone it.

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub age: u8,
}
