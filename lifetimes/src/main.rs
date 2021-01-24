// 🛑 This program will fail with the following compile error
//
//   |         result = longest(x, &y);
//   |                             ^^ borrowed value does not live long enough
//   |     }
//   |     - `y` dropped here while still borrowed
//   |
//   |     println!("{}", result)
//   |                    ------ borrow later used here
fn main() {
    let x = "this is x";
    let result;
    {
        let y = String::from("this is y");
        // `result` is borrowing `y`.
        result = longest(x, &y);
    }
    // `result` is in scope here, but `y` is not.

    // Borrow of `y` is used here, but `y` has been dropped.
    println!("{}", result)
}

// 🛑 If 'a is removed from y:
//
//    | fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//    |                               ---- help: add explicit lifetime `'a` to the type of `y`: `&'a str`
// ...
//    |         y
//    |         ^ lifetime `'a` required
//
// 🛑 If 'a is removed from return type:
//
//    | fn longest<'a>(x: &'a str, y: &'a str) -> &str {
//    |                   -------     -------     ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider using the `'a` lifetime
//    |
//    | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    |                                           ^^^
//
// 🛑 If y has separate lifetime 'b:
//
//    | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//    |                                   -------     -------
//    |                                   |
//    |                                   this parameter and the return type are declared with different lifetimes...
// ...
//    |         y
//    |         ^ ...but data from `y` is returned here
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
