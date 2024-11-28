fn copy_demo() {
    // Wrong!
    // Since String type is unsure type
    // Rust will alloc it in heap
    // While copy data in heap will move the ownership
    // So it fails
    // let s1 = String::from("hello world");
    // let s2 = s1;
    // println!("{}, world!", s1);
    // println!("{}", s2);

    // Yes!
    // Since &str is sure type
    // Rust will alloc it on stack
    // And it's ok to copy data from stack
    let s3 = "hello world";
    let s4 = s3;
    println!("{}", s3);
    println!("{}", s4);

    // Yes!
    // By using clone method will also make it save
    // Because it's deep copy
    // As cost, it's quite slow
    let s5 = String::from("hello world");
    let s6 = s5.clone();
    println!("{}", s5);
    println!("{}", s6);
}