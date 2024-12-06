fn main() {
    let mut x = 5;
    { //Scope for mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y += 1;      // Modifying x through y is allowed
    }
    println!("x = {}", x); // x is 6

    let a = 10;
    let b = &a; // b is an immutable reference to a
    // *b += 1;  // This is not allowed, cannot modify an immutable reference
    let mut c = a; // Create a mutable copy for modification
    c += 1;
    println!("c = {}", c); // c is 11
}
