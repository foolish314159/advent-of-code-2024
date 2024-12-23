use day7::Operator;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Day 1 part 1: {}", day1::total_distance("data/day1.txt"));
    println!("Day 1 part 2: {}", day1::similarity_score("data/day1.txt"));

    println!(
        "Day 2 part 1: {}",
        day2::safe_reports("data/day2.txt", false)
    );
    println!(
        "Day 2 part 2: {}",
        day2::safe_reports("data/day2.txt", true)
    );

    println!(
        "Day 3 part 1: {}",
        day3::sum_of_valid_muls("data/day3.txt", false)
    );
    println!(
        "Day 3 part 2: {}",
        day3::sum_of_valid_muls("data/day3.txt", true)
    );

    println!("Day 4 part 1: {}", day4::xmas_count("data/day4.txt"));
    println!("Day 4 part 2: {}", day4::x_mas_count("data/day4.txt"));

    println!(
        "Day 5 part 1: {}",
        day5::sum_of_correct_update_middle_pages("data/day5rules.txt", "data/day5updates.txt")
    );
    println!(
        "Day 5 part 2: {}",
        day5::sum_of_reordered_middle_pages("data/day5rules.txt", "data/day5updates.txt")
    );

    println!(
        "Day 6 part 1: {}",
        day6::distinct_positions("data/day6.txt")
    );

    println!(
        "Day 6 part 2: {}",
        day6::obstacles_causing_a_loop("data/day6.txt")
    );

    println!(
        "Day 7 part 1: {}",
        day7::calibration_result("data/day7.txt", &[Operator::Add, Operator::Mul])
    );

    println!(
        "Day 7 part 2: {}",
        day7::calibration_result(
            "data/day7.txt",
            &[Operator::Add, Operator::Mul, Operator::Concat]
        )
    );
}
