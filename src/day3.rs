use std::fs::read_to_string;

pub fn sum_of_valid_muls(filename: &str, check_do_dont: bool) -> i32 {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    let mut do_it = true;
    let mut sum = 0;
    let mut current = 0;
    while current < input.len() {
        // Check if there are any more possible instructions starting with "mul("
        let next_mul_start = match input[current..].find("mul(") {
            Some(idx) => idx,
            None => return sum,
        };

        if check_do_dont {
            // Check for the most recent do/don't call and update it
            let do_dont = get_most_recent_do_dont(&input[current..current + next_mul_start]);
            match do_dont {
                DoDontInstr::Do => do_it = true,
                DoDontInstr::Dont => do_it = false,
                DoDontInstr::NotFound => (),
            }
        }

        if let Some(mul) = get_next_mul(&input[current..]) {
            // Found a valid mul instruction
            current += mul.end + 1;
            if do_it {
                sum += mul.op_1 * mul.op_2;
            }
        } else {
            // The last mul wasn't valid, check the next possible candidate
            current += next_mul_start + 4;
        }
    }

    sum
}

struct MulInstr {
    end: usize,
    op_1: i32,
    op_2: i32,
}

enum DoDontInstr {
    Do,
    Dont,
    NotFound,
}

fn get_most_recent_do_dont(input: &str) -> DoDontInstr {
    let do_start = input.rfind("do()");
    let dont_start = input.rfind("don't()");

    return match (do_start, dont_start) {
        (Some(_), None) => DoDontInstr::Do,
        (None, Some(_)) => DoDontInstr::Dont,
        (Some(do_idx), Some(dont_idx)) => {
            if do_idx > dont_idx {
                DoDontInstr::Do
            } else {
                DoDontInstr::Dont
            }
        }
        (None, None) => DoDontInstr::NotFound,
    };
}

fn get_next_mul(input: &str) -> Option<MulInstr> {
    let start = input.find("mul(")?;
    let offset = input[(start)..].find(")")?;
    let end = start + offset;

    let mul = &input[start + 4..end];
    let mut ops = mul.split(",");
    let ops_1 = ops.next()?;
    let ops_2 = ops.next()?;

    let op_1 = ops_1.parse::<i32>().ok()?;
    let op_2 = ops_2.parse::<i32>().ok()?;

    Some(MulInstr { end, op_1, op_2 })
}
