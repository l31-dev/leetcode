pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let nums_len: usize = nums.len();
    let mut max_sum: i64 = i64::MIN;

    for i in 0..nums_len {
        let mut subarray_sum: i64 = 0;

        for j in i..nums_len {
            subarray_sum += nums[j] as i64;

            if ((nums[j] - nums[i]).abs() == k) && (subarray_sum > max_sum) {
                max_sum = subarray_sum;
            }
        }
    }

    max_sum
}
