use std::collections::{HashMap, HashSet};

pub fn array_rs() {
    let the_list = vec![1, 2, 3];
    let added: Vec<_> = the_list.iter().map(|x| x + 1).collect();

    println!("{:?}", added);
}

fn manual_array_rs() {
    // manually collect
    let the_list = vec![1, 2, 3];
    let mut the_iterator = the_list
        .iter() // turn into iterator (thing that produces next)
        .map(|x| x + 1); // the iterator does this thing

    let mut new_vector = vec![];

    // runs until an undefined is returned
    // while Some() returns something, do this
    while let Some(x) = the_iterator.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);
}

/*
in rust, we work a lot with iterators, and its very powerful
*/

/*
because you tell rust what you want by defining the type
after foo:, it knows what you actually want and calls
the type of iterator associated with its type
in this case, a HashSet is a new Set() (ts) equivalent,
when you turn a call a hashset iterator over a vector
your vector will only have UNIQUE VALUES
 */
pub fn unique_array() {
    let foo: HashSet<_> = vec!["this", "what", "is", "nani", "kore", "is", "this"] // Array.from(new Set([what is this nani kore is this]))
        .into_iter()
        .collect();

    println!("{:?}", foo);
}

pub fn hash_set() {
    let foo: HashMap<_, _> = vec!["nani", "what", "is", "this", "kore", "is", "this"]
        .into_iter()
        .enumerate() // adds the index
        .map(|(i, item)| (item, i)) // keys must be unique, so if you flip them, this means only the latest key will be kept (e.g. this: 6)
        .collect(); // this is the point where it 'executes' all the above code

    println!("{:?}", foo); // {"kore": 4, "what": 0, "is": 5, "nani": 3, "this": 6}
}

pub fn add() {
    let value: usize = vec![1, 2, 3].iter().sum();

    println!("{:?}", value)
}
