use std::cmp::Ordering;

pub type Pair = (u32, u32);

pub type Triplets = (u32, u32, u32);

pub fn find_pair_with_sum(list: &[u32], sum: u32) -> Pair {
    let set_of_numbers = crate::utils::load_into_set(list);

    for n in list {
        if set_of_numbers.contains(&(sum - n)) {
            return (*n, sum - *n);
        }
    }

    (0, 0)
}

pub fn find_triplets_with_sum(list: &[u32], sum: u32) -> Triplets {
    let mut sorted = list.to_vec();
    sorted.sort_unstable();

    for i in 0..sorted.len() {
        let mut l = i + 1;
        let mut r = sorted.len() - 1;

        while l < r {
            let actual = sorted[i] + sorted[l] + sorted[r];
            match sum.cmp(&actual) {
                Ordering::Greater => l += 1,
                Ordering::Less => r -= 1,
                Ordering::Equal => return (sorted[i], sorted[l], sorted[r]),
            }
        }
    }

    (0, 0, 0)
}
