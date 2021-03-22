// lifetimes2.rs
// Make me compile! Execute `rustlings help lifetimes2` for hints :)

// I AM NOT DONE

fn main() {
    let short = "a short string";
    let longer = "a longer string";
    let most_longest = "the longest string here";

    println!("The longer of these two strings:");
    println!(" - {}", short);
    println!(" - {}", longer);
    println!("...is {}", longest(short, longer));

    println!("---");

    println!("The longer of these two strings:");
    println!(" - {}", longer);
    println!(" - {}", most_longest);
    println!("...is {}", longest(longer, most_longest));
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
