use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn calc_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use crate::util::calc_hash;
    #[test]
    fn hash_the_same_input() {
        let input1 = "test";

        let result1 = calc_hash(&input1);
        let result2 = calc_hash(&input1);

        assert_eq!(result1, result2);
    }

    #[test]
    fn hash_different_input() {
        let input1 = "test";
        let input2 = 123;

        let result1 = calc_hash(&input1);
        let result2 = calc_hash(&input2);

        assert_ne!(result1, result2);
    }
}
