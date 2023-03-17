fn main() {
    part_1();
}

fn part_1() {
    std::fs::read_to_string(std::env::args().nth(1).expect("File Name Not Provided."))
        .expect("File Path is not correct")
        .lines()
        .for_each(|line| println!("{}", line));
}
