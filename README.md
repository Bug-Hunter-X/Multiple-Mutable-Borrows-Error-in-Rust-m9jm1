# Multiple Mutable Borrows in Rust
This repository demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  The code in `bug.rs` will not compile due to the borrow checker preventing data races.
The solution in `bugSolution.rs` shows how to correctly handle this situation using techniques like cloning or using a mutable reference within a block.