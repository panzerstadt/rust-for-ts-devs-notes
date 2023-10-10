fn all_lines() {
    let file = std::fs::read_to_string("lines");
    match file {
        Ok(filestr) => {
            // filestr.lines().for_each(|x| println!("{:?}", x)); // also can
            filestr
                .split("\n")
                .into_iter()
                .for_each(|x| println!("{:?}", x));
        }
        Err(msg) => {
            println!("Error: {:?}. Ending Program.", msg.to_string())
        }
    }
}

fn every_other_line() {
    let file = std::fs::read_to_string("lines");
    match file {
        Ok(filestr) => {
            filestr
                .lines()
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 0)
                .for_each(|(_, x)| println!("{}", x));
        }
        Err(msg) => {
            println!("Error: {:?}. Ending Program.", msg.to_string())
        }
    }
}

/*
in order
- every other line
- skip first 2
- print 2
*/
fn custom() {
    let file = std::fs::read_to_string("lines");
    match file {
        Ok(filestr) => {
            filestr
                .lines()
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 0)
                .skip(2)
                .take(2)
                .for_each(|(_, x)| println!("{}", x));
        }
        Err(msg) => {
            println!("Error: {:?}. Ending Program.", msg.to_string())
        }
    }
}
