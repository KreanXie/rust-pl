fn fn_and_ownership() {
    let mut s = String::from("hello");

    take_ownership(s);

    let mut i = 1024;

    take_it(i);

    let result = get_result();
}

// In here, this function has taken the ownership of some_string
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

// In here, this function just copy the value of val, and print it
fn take_it(val: i32) {
    println!("{}", val);
}

// In here, this function create a var s, and give it's ownership to the caller
fn get_result() -> String {
    let s = String::from("hello");
    s
}