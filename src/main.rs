use std::fs;

fn main() {
    let path: &str = "input.cav";

    // This extra binding to non mutable content is needed as expect() consumes the value and split()
    // has a lifetime specifier which would point to a temporary object (to
    // the x value of Some(x)) by expect().
    // We can use the object in that statement, but after the
    // statement is finished, it will be discarded. Hence, we need to bind
    // the result of expect to a variable which then can be used further down
    // the road.
    //
    // So this would not work
    // let mut content = fs::read_to_string(&path)
    //     .expect("Could not read file")
    //     .split(',');
    //
    // Instead we use the intermediate let binding to hold the Some() value
    // which is returned by expect().
    let content = fs::read_to_string(&path).expect("Could not read file");
    let mut content = content.split(',');

    // The number of nodes
    let amount: u32 = content
        .next()
        .unwrap_or_else(|| panic!("File is not properly formatted. Expecting something here!"))
        .parse::<u32>()
        .unwrap_or_else(|count| panic!("Could not parse {} to u32", count));

    // Parse the coordinates
    // With using unwrap() here, we are assuming that the input file is
    // correctly structered.
    let mut nodes: Vec<(i32, i32)> = Vec::with_capacity((amount / 2) as usize);
    for _ in 0..amount {
        nodes.push((
            content.next().unwrap().parse().unwrap(),
            content.next().unwrap().parse().unwrap(),
        ));
    }

    // Read [amount X amount] matrix
    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(amount as usize);
    for _ in 0..amount {
        matrix.push(
            content
                .by_ref()
                .take(amount as usize)
                .map(|arg| arg.parse::<u8>().unwrap())
                .collect(),
        );
    }

    println!("Number of nodes: {}", amount);
    println!("Nodes: {:?}", nodes);
    println!("Matrix: {:?}", matrix);

    // for element in content.split(',') {
    //     println!("{}", element);
    // }
}
