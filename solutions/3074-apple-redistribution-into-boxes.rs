pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let total_apples: i32 = apple.iter().sum();
    let mut capacities = capacity.clone();
    capacities.sort_by(|a, b| b.cmp(a));

    let mut boxes_used = 0;
    let mut remaining_apples = total_apples;

    for cap in capacities {
        remaining_apples -= cap;
        boxes_used += 1;

        if remaining_apples <= 0 {
            break;
        }
    }

    boxes_used
}
