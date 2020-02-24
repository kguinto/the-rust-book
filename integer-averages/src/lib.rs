use std::collections::HashMap;
use std::f32::NAN;

pub fn get_mean(nums: &Vec<i32>) -> f32 {
    if nums.len() == 0 {
        return NAN;
    }

    let mut sum = 0;

    for num in nums.iter() {
        sum += *num;
    }

    sum as f32 / nums.len() as f32
}

pub fn get_median(nums: &Vec<i32>) -> f32 {
    if nums.len() == 0 {
        return NAN;
    }

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    match sorted_nums.len() % 2 == 0 {
        true => {
            let midpoint = sorted_nums.len() / 2;

            (sorted_nums[midpoint - 1] + sorted_nums[midpoint]) as f32 / 2.0
        }
        false => sorted_nums[sorted_nums.len() / 2] as f32,
    }
}

pub fn get_mode(nums: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    let mut mode = (0, 0); // (num, max)

    for num in nums.iter() {
        let count = counts.entry(num).or_insert(0);
        *count += 1;

        if mode.1 < *count {
            mode = (*num, *count)
        }
    }

    mode.0
}
