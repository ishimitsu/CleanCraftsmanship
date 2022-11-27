pub fn sort(mut vec: Vec<i32>) -> Vec<i32> {
    return vec;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![0], sort(vec![0]));
        assert_eq!(vec![1], sort(vec![1]));
        assert_eq!(vec![1, 2], sort(vec![1, 2]));                
    }
}
