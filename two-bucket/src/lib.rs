mod bucket;
pub mod buckets;

use buckets::Buckets;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let (buckets, illegal) = match start_bucket {
        Bucket::One => {
            let mut buckets = Buckets::new(capacity_1, capacity_2);
            let mut illegal = Buckets::new(capacity_1, capacity_2);
            (buckets.fill_left(), illegal.fill_right())
        }
        Bucket::Two => {
            let mut buckets = Buckets::new(capacity_1, capacity_2);
            let mut illegal = Buckets::new(capacity_1, capacity_2);
            (buckets.fill_right(), illegal.fill_left())
        }
    };
    let (moves, buckets) = next(&buckets, goal, &illegal).unwrap();
    let (left, right) = buckets.get_contents();
    let (goal_bucket, other_bucket) = if left == goal {
        (Bucket::One, right)
    } else {
        (Bucket::Two, left)
    };
    BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    }
}

fn next_helper(
    buckets: &Buckets,
    list: &[Buckets],
    goal: u8,
    illegal: &Buckets,
    count: u8,
) -> Option<(u8, Buckets)> {
    if buckets.contains(goal) {
        return Some((count, buckets.clone()));
    }
    if buckets == illegal {
        return None;
    }

    let output = vec![
        buckets.empty_left(),
        buckets.empty_right(),
        buckets.fill_left(),
        buckets.fill_right(),
        buckets.transfer_left(),
        buckets.transfer_right(),
    ];

    let results: Vec<(u8, Buckets)> = output
        .into_iter()
        .filter(|item| !list.contains(&item))
        .filter_map(|item| {
            let mut new_list = list.to_vec();
            new_list.push(item.clone());
            next_helper(&item, &new_list, goal, illegal, count + 1)
        })
        .collect();

    if results.is_empty() {
        return None;
    }
    results.into_iter().min_by(|x, y| (x.0).cmp(&y.0))
}

pub fn next(buckets: &Buckets, goal: u8, illegal: &Buckets) -> Option<(u8, Buckets)> {
    let list = &[buckets.clone()];
    next_helper(&buckets, list, goal, &illegal, 1)
}
