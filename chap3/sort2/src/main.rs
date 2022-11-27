pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 { return vec; }

    if vec.len() == 2 { 
        let fst = vec[0];
        let sec = vec[1];
        if fst > sec { return vec![sec, fst]; }
        else { return vec![fst, sec]; }        
    }

    else {
        let middle = vec[0];
        let lessers = vec.iter().filter(|&x| x < &middle).cloned().collect();
        let greaters = vec.iter().filter(|&x| x > &middle).cloned().collect();

        let mut result = vec![0; 0];
        result.append(&mut sort(lessers));
        result.push(middle);
        result.append(&mut sort(greaters));
        return result;
    }
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![1], sort(vec![1]));
        assert_eq!(vec![1, 2], sort(vec![2, 1]));
        assert_eq!(vec![1, 2, 3], sort(vec![2, 1, 3]));
        assert_eq!(vec![1, 2, 3], sort(vec![1, 3, 2]));
        assert_eq!(vec![1, 2, 3], sort(vec![3, 2, 1]));
        assert_eq!(vec![1, 2, 3, 4], sort(vec![1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4], sort(vec![2, 1, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4], sort(vec![4, 3, 2, 1]));
    }
}
