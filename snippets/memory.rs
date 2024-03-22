// Write a code snippet to show rust's memory safe use concept of ownerships
// so write a code that will throw error ( will not in c )

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}

// Error: cannot borrow `s` as mutable more than once at a time

fn returnAPointer() {
    let s = String::from("hello");
    &s
}

fn main() {
    let r = returnAPointer();
    println!("{}", r);
}

// Error: missing lifetime specifier
// Because the return value of the function is a reference to a value that goes out of scope at the end of the function.
