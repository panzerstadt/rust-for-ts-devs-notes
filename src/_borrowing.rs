fn panics_one() {
    let mut items: Vec<Item> = vec![Item { count: 10 }, Item { count: 12 }];
    let first = items.get_mut(0);
    println!("{:?}", first);
    items.pop();
    let second = items.get_mut(1).unwrap(); // valid rust code, will panic ONLY at runtime, because there's no second item
    println!("{:?}", second.count); // after panic, code never gets here
}

fn forgo_safety() {
    let mut items: Vec<Item> = vec![Item { count: 10 }, Item { count: 12 }];
    let first: Item = items.get(0).unwrap().clone();
    println!("{:?}", first);
    items.pop();
    // items.push(first);
    unsafe {
        let second = items.get_mut(1).unwrap_unchecked(); // doesn't panic if none
        println!("{:?}", second.count); // so it errors here
    }
}
