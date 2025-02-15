# Rust Iterator Exhaustion Bug

This repository demonstrates a common error in Rust: panicking due to iterator exhaustion. The `bug.rs` file contains code that iterates over a vector and attempts to access elements beyond its end, causing a runtime panic. The `bugSolution.rs` file shows how to handle this by checking for the end of the iterator before attempting to access further elements.