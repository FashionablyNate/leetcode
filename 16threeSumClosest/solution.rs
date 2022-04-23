pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {

    let nums_len: usize = nums.len();
    let mut sum: i32;
    nums.sort();

    let mut closest: Option<i32> = None;

    for (start, &x) in nums.iter().enumerate() {
        
        // we set the left pointer just right of x
        let mut left: usize = start + 1;

        // we set the right pointer at the end
        let mut right: usize = nums_len - 1;

        while left < right {
            sum = nums[left] + nums[right] + x;
            // if the sum is bigger than the target, we need a smaller right
            // value
            if sum > target {
                right -= 1;
            // if the sum is smaller than the target, we need a bigger left
            // value
            } else if sum < target {
                left += 1;
            // otherwise...
            } else {
                return sum;
            }

            if let Some(val) = closest {
                if (target - sum).abs() < (target - val).abs() {
                    closest = Some(sum);
                }
            } else {
                closest = Some(sum);
            }
        }
    }
    return closest.unwrap();
}

fn main() {
    println!("Ans: {}", three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!("Ans: {}", three_sum_closest(vec![0, 0, 0], 1));
    println!("Ans: {}", three_sum_closest(vec![1, 1, 1, 1], 0));
    println!("Ans: {}", three_sum_closest(vec![1, 1, 1, 0], -100));
    println!("Ans: {}", three_sum_closest(vec![0, 2, 1, -3], 1));
}