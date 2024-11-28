// What if I want to call a function
// but don't want to give it a variable's ownership
fn borrow_demo() {
    let s1 = String::from("hello");

    let length = get_length(&s1);

    println!("The length of '{}' is {}.", s1, length);

    // Can I change the borrowed variable?
    // No
    do_something(&s1);

    // If s1 is set mutable,
    // It works
    let mut s2 = s1;
    do_something_again(&mut s2);
}

// With '&', function can receive variable without its ownership,
// Which means the data in the heap wouldn't be freed as the function ends
fn get_length(s: &String) -> usize {
    s.len()
}

fn do_something(s: &String) {
    // This won't work
    // Cuz s is immutable
    // s.push_str(", world!");
}

fn do_something_again(s: &mut String) {
    // This work
    s.push_str(", world");
}