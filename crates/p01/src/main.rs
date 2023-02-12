use itertools::{self, Itertools};

static INPUT: &str = include_str!("../input");

fn main() {
    let r = solve(INPUT);

    println!("{r}");
}

fn solve(input: &'static str) -> usize {
    let all = input
        .lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .collect_vec();

    let q = (0..all.len() - 60).fold(0, |mut acc, n| {
        let avg = &all[n..n + 60].iter().sum::<usize>() / 60;

        if !(1500..=1600).contains(&avg) {
            acc += 1;
        }

        acc
    });

    q
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn main() {
        assert_eq!(super::solve(INPUT), 10);
    }
}
