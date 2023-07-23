fn main() {
    let foo: HashMap<_, _> = vec!["what", "is", "this", "nani", "kore", "is", "this"]
        .into_iter()
        .enumerate() // adds the index
        .map(|(i, item)| (item, i)) // keys must be unique, so if you flip them, this means only the latest key will be kept (e.g. this: 6)
        .collect(); // this is the point where it 'executes' all the above code

    println!("{:?}", foo);
}
