pub fn array_rs() {
    let the_list = vec![1, 2, 3];
    let added: Vec<_> = the_list.iter().map(|x| x + 1).collect();

    println!("{:?}", added);
}
