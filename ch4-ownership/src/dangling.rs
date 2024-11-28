fn dangling() {
    // let r = dangle();
}

// If this function works, s will be freed after the function ends,
// but the ptr still returns,
// so there is a ptr point no data, this is terrible,
// so this function doesn't work
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }