use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines_it = contents.lines();

    let lines: Vec<&str> = lines_it.collect();

    let mut sums: Vec<i32> = vec![];

    {
        let mut sum = 0;
        for line in lines.iter() {
            if line.trim().is_empty() {
                if sum != 0 {
                    sums.push(sum);
                    sum = 0;
                }
            } else {
                let n: i32 = line.trim().parse().unwrap();
                sum += n;
            }
        }
    }

    sums.sort();
    sums.reverse();

    let mut top3 = 0;

    for i in 0..3 {
        top3 += sums[i];
    }

    println!("{}", top3);
}
