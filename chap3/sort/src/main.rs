pub fn sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() > 1 {
        if vec[0] > vec[1] {
            let tmp: i32 = vec[0];
            vec[0] = vec[1];
            vec[1] = tmp;
        }
    }

    return vec;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn sorted() {
        let expect = vec![1, 2];
        assert_eq!(expect, sort(vec![2, 1]));
    }
}
