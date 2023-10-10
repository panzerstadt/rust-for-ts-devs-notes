fn practice(values: &Vec<usize>, index: usize) -> usize {
    match values.get(index) {
        // if value exists at index in nums, return it multiplied by 5
        Some(val) => val * 5,
        // if there is no value, return index multiplied by 5
        None => index * 5,
    }
}

// equivalent
fn practice(values: &Vec<usize>, index: usize) -> usize {
    return values.get(index).unwrap_or(&index) * 5;
}

fn main() {
    let vector = vec![3, 2, 1];
    println!("{:?}", practice(&vector, 0));
    println!("{:?}", practice(&vector, 5));
}
