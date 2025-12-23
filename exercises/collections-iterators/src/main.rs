fn sum(nums: &[i32]) -> i32 {
    nums.iter().copied().sum()
}

fn max(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().max()
}

fn contains(nums: &[i32], target: i32) -> bool {
    nums.iter().any(|n| *n == target)
}

fn main() {
    let nums: Vec<i32> = vec![10, 20, 30, 40];

    let last_index: usize = nums.len() - 1;

    println!("len = {}", nums.len());
    println!("first = {}", nums[0]);
    println!("last = {}", nums[last_index]);

    let sum: i32 = nums.iter().copied().sum();
    println!("sum = {}", sum);

    for n in nums.iter() {
        println!("{}", *n);
    }

    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("doubled = {:?}", doubled);

    let more_nums = vec![10, 21, 30, 41, 50];

    let has_even = more_nums.iter().any(|n| n % 2 == 0);
    println!("has even = {}", has_even);

    let first_over_40 = more_nums.iter().find(|n| **n > 40);
    println!("first over 40 = {:?}", first_over_40);

    let maybe = more_nums.get(3);
    match maybe {
        Some(n) => println!("found: {}", n),
        None => println!("no value at that index"),
    }
}
