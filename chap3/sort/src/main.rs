pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    return vec![0; 1];
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn sorted() {
        let expect = vec![0; 1];
        let new = vec![1; 1];
        let result = sort(new);
        assert_eq!(expect, result);
    }
}
