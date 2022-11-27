pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 { return vec; }

    if vec.len() == 2 {
        let fst = vec[0];
        let sec = vec[1];
        if fst > sec { return vec![sec, fst]; }
        else { return vec![fst, sec]; }
    }

    return vec;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![0], sort(vec![0]));
        assert_eq!(vec![1], sort(vec![1]));
        assert_eq!(vec![1, 2], sort(vec![2, 1]));
        assert_eq!(vec![1, 2, 3], sort(vec![1, 2, 3]));
    }
}
