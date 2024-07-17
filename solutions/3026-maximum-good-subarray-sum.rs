use std::collections::VecDeque;

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut max_sum = 0;
    let mut min_deque: VecDeque<usize> = VecDeque::new();
    let mut max_deque: VecDeque<usize> = VecDeque::new();
    let mut current_sum = 0;

    let mut start = 0;
    for end in 0..nums.len() {
        current_sum += nums[end] as i64;

        while let Some(&index) = min_deque.back() {
            if nums[index] >= nums[end] {
                min_deque.pop_back();
            } else {
                break;
            }
        }
        min_deque.push_back(end);

        while let Some(&index) = max_deque.back() {
            if nums[index] <= nums[end] {
                max_deque.pop_back();
            } else {
                break;
            }
        }
        max_deque.push_back(end);

        while !min_deque.is_empty()
            && !max_deque.is_empty()
            && (nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()]).abs() == k
        {
            max_sum = max_sum.max(current_sum);
            if *max_deque.front().unwrap() == start {
                max_deque.pop_front();
            }
            if *min_deque.front().unwrap() == start {
                min_deque.pop_front();
            }
            current_sum -= nums[start] as i64;
            start += 1;
        }
    }

    max_sum
}
