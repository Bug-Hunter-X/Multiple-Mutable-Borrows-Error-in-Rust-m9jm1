fn main() {
    let mut x = 5;
    { // Use a block to limit the scope of mutable references
        let y = &mut x;
        *y += 1;
    }
    { // Use another block
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x); //x will be 7
} 