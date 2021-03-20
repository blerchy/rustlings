// lifetimes1.rs
// Make me compile, only by changing the position of line 11!
// Execute `rustlings hint lifetimes1` for hints :)

// I AM NOT DONE

fn main() {
    let r;

    {
        let x = 5; // Move this line elsewhere
        r = &x;
    }

    println!("r: {}", r);
}
