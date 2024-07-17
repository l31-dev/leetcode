pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut times_to_reach_city = vec![];

    for i in 0..dist.len() {
        let time = (dist[i] + speed[i] - 1) / speed[i];
        times_to_reach_city.push(time);
    }

    times_to_reach_city.sort();

    let mut eliminated = 0;

    for (elapsed_time, &v) in times_to_reach_city.iter().enumerate() {
        if elapsed_time < v as usize {
            eliminated += 1;
        } else {
            break;
        }
    }

    eliminated
}
