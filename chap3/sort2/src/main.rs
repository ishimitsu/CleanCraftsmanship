pub fn sort(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 { return vec; }

    if vec.len() == 2 { 
        let fst = vec[0];
        let sec = vec[1];
        if fst > sec { return vec![sec, fst]; }
        else { return vec![fst, sec]; }        
    }

    if vec.len() == 3 {}
    let fst = vec[0];
    let sec = vec[1];
    let thd = vec[2];

    let mut lessers = vec![0; 0];
    let mut greaters = vec![0; 0];

    if fst < sec { lessers.push(fst) }
    if thd < sec { lessers.push(thd) }
    if fst > sec { greaters.push(fst) }
    if thd > sec { greaters.push(thd) }

    let mut result = vec![0; 0];
    result.append(&mut lessers);
    result.push(sec);
    result.append(&mut greaters);

    return result;
}

#[cfg(test)]
mod sort_test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(vec![1], sort(vec![1]));
        assert_eq!(vec![1, 2], sort(vec![2, 1]));
        assert_eq!(vec![1, 2, 3], sort(vec![2, 1, 3]));
    }
}
