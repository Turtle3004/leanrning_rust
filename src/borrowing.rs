// Mutable borrow allows mutation and doesnt allow aliasing.
// Shared borrow allows aliasing but not mutation.
// Borrowing prevents use-after-free.

pub fn run() {
    let mut v = Vec::new();
    push(&mut v); // Mutatble borrow.
    read(&v); // Shared borrow.
}

fn push(v: &mut Vec<i32>) {
    v.push(100);
}

fn read(v: &Vec<i32>) {
    println!("{:?}", v);
}
