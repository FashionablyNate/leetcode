pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut ret: Vec<Vec<i32>> = Vec::new();
    let nums_len: usize = nums.len();

    if nums.is_empty() { return ret; }
    if nums_len == 1 { return ret; }

    nums.sort();

    for x in 0..(nums_len - 2) {
        // since this is sorted, if the same number is behind I throw out
        // the index
        if x > 0 && nums[x] == nums[x - 1] { continue; }
        if nums[x] > 0 { return ret; }

        let mut left = x + 1;
        let mut right = nums_len - 1;

        while left < right {
            let i = nums[left];
            let j = nums[right];
            let k = nums[x];
            let sum = i + j + k;

            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                ret.push(vec![i, j, k]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            }
        }
    }
    return ret;
}

fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1 -4]))
}