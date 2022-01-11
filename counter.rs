use std::collections::HashMap;

type CounterValue = i64;

pub trait Counter<T> {
    fn counter(&mut self) -> HashMap<T, CounterValue>;
}

impl<I: Iterator<Item = T>, T> Counter<T> for I
where
    T: Eq + std::hash::Hash,
{
    fn counter(&mut self) -> HashMap<T, CounterValue> {
        let mut res = HashMap::new();
        for k in self {
            *res.entry(k).or_insert(0) += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_works() {
        let arr = vec![1, 2, 3, 5, 5, 1_000_000_007];
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 1);
        map.insert(3, 1);
        map.insert(5, 2);
        map.insert(1_000_000_007, 1);
        let res = arr.iter().cloned().counter();
        assert_eq!(res, map);

        let arr = "Hello,world!".chars().collect::<Vec<_>>();
        let mut map = HashMap::new();
        map.insert('H', 1);
        map.insert('e', 1);
        map.insert('l', 3);
        map.insert('o', 2);
        map.insert(',', 1);
        map.insert('w', 1);
        map.insert('r', 1);
        map.insert('d', 1);
        map.insert('!', 1);
        let res = arr.iter().cloned().counter();
        assert_eq!(res, map);
    }
}
