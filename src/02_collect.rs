fn main() {
    let data = vec![1, 2, 3];
    let mut added = data.iter().map(|item| item + 1);

    // this is the equivalent to .collect(), we are just looping through the iterator,
    // and putting them into a new vector
    let mut new_vector = vec![];
    while let Some(x) = added.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);
}
