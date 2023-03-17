fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("file path need's to be given.");
    part_1(&file_name);
    part_2(&file_name);
}

// Read's the input from the file
fn part_1(file_name: &String) {
    println!(
        "{}",
        std::fs::read_to_string(file_name)
            .expect("file path is not correct")
            .split("\n\n")
            .flat_map(|group| {
                let lines = group.lines();
                let numbers = lines
                    .flat_map(|line| line.parse::<i32>())
                    .reduce(|acc, e| acc + e);
                numbers
            })
            .max()
            .unwrap_or(0)
    );
}
fn part_2(file_name: &String) {
    let file = std::fs::read_to_string(file_name);

    let mut values = file
        .expect("file path is not correct")
        .split("\n\n")
        .map(|group| {
            let lines = group.lines();
            let number = lines
                .map(|line| line.parse::<i32>().unwrap())
                .reduce(|acc, e| acc + e)
                .unwrap();
            number
        })
        .collect::<Vec<i32>>();
    values.sort();

    values.reverse();
    println!(
        "{}",
        values.into_iter().take(3).reduce(|acc, e| acc + e).unwrap()
    );
}
