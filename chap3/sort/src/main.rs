pub fn sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() > 1 {
        for idx in 0..vec.len() {
            let next_idx = idx + 1;
            if next_idx == vec.len() { break };
            if vec[idx] > vec[next_idx] {
                let tmp: i32 = vec[idx];
                vec[idx] = vec[next_idx];
                vec[next_idx] = tmp;
            }
        }
    }

    return vec;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn sorted() {
        let expect = vec![1, 2, 3];
        assert_eq!(expect, sort(vec![2, 3, 1]));
    }
}
