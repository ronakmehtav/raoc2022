fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("file path need's to be given."),
    );

    let max = file
        .expect("file path is not correct")
        .split("\n\n")
        .map(|group| {
            let lines = group.lines();
            let numbers = lines
                .map(|line| line.parse::<i32>().unwrap())
                .reduce(|acc, e| acc + e)
                .unwrap();
            numbers
        })
        .max()
        .unwrap();

    println!("{}", max);
}
fn part_2() {
    let file = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("file path need's to be given."),
    );

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
