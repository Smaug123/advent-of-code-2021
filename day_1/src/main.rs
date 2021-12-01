use day_1::day_1;

fn main() {
    let input = day_1::input();
    println!("part 1 => {}", day_1::part_1(&mut input.iter().copied()));
    println!("part 2 => {}", day_1::part_2(&input));
}
