pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut max: i32 = 0;

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (nums[i] - nums[j]) == k {
                let a: i32 = nums[i..j].iter().sum();
                if a > max {
                    max = a;
                }
            }
        }
    }

    i64::from(max)
}
