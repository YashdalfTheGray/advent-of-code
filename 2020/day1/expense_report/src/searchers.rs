pub type Pair = (u32, u32);

pub fn find_pair_with_sum(list: &[u32], sum: u32) -> Pair {
    let set_of_numbers = crate::utils::load_into_set(list);

    for n in list {
        if set_of_numbers.contains(&(sum - n)) {
            return (*n, sum - *n);
        }
    }

    (0, 0)
}
