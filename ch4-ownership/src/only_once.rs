pub fn only_once() {
    // Wrong!
    // The variable can't be borrowed at mutable,
    // more than once at a time(in one scope)
    // In case of data risk
    // let mut s = String::from("hello");
    // let s1 = &mut s;
    // let s2 = &mut s;
    // println!("{}, {}", s1, s2);

    // Wrong!
    // If there is already a borrowed, mutable variable,
    // There shouldn't be another borrowed, immutable variable
    // let mut s = String::from("hello");
    // let s1 = &mut s;
    // let s2 = s;
    // println!("{}, {}", s1, s2);

    // Yes!
    // Multi borrowed, immutable variable is legal
    let mut s = String::from("hello");
    let s1 = & s;
    let s2 = &mut s;
    println!("{} {}", s1, s2);
}