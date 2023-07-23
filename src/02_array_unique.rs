use std::collections::HashSet;

fn main() {
    let foo: HashSet<_> = vec!["what", "is", "this", "nani", "kore", "is", "this"] // Array.from(new Set([what is this nani kore is this]))
        .into_iter()
        .collect();

    println!("{:?}", foo);
}
