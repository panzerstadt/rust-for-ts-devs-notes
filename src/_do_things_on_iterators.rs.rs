use std::collections::HashMap;

fn main() {
    let foo: HashMap<_, _> = vec!["what", "is", "this", "nani", "kore", "is", "this"]
        .into_iter()
        .enumerate() // adds the index (kinda like confluence, add number column). column is in front of value
        .map(|(i, item)| (item, i)) // keys must be unique, so if you flip them, this means only the latest key will be kept (e.g. this: 6)
        .collect(); // this is the point where it 'executes' all the above code

    println!("{:?}", foo);
}

// kinda like how flooent is structured, you turn everything into an iterator, you iterate on it, and then you collect them at the end
