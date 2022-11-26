pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    return vec;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn sorted() {
        let expect = vec![0; 1];
        assert_eq!(expect, sort(vec![0; 1]));
    }
}
