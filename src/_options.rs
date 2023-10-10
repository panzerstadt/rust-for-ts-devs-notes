// when you wanna always return number
fn multiply_five(input: Option<usize>) -> usize {
    return input.unwrap_or(0) * 5;
}
// when you wanna return number OR undefined
fn multiply_five(input: Option<usize>) -> Option<usize> {
    let input = input?; // if input = None, return None (? is the shortform)
    return Some(input * 5);
}
fn multiply_five2(input: Option<usize>) -> Option<usize> {
    return Some(input? * 5);
}
fn multiply_five3(input: Option<usize>) -> Option<usize> {
    return input.unwrap;
}

fn main() {
    println!("{:?}", multiply_five(Some(2)));
    println!("{:?}", multiply_five(None));
}
