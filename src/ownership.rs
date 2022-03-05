// Ownership allows mutation but it doesn't allow aliasing.
// It prevents double-free

pub fn run() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    take(v); // ownership tranferred to take(), scope of v ends here.
             //v.push(3); -> error, use of moved value.
}

fn take(v: Vec<i32>) {
    println!("{:?}", v);
}
