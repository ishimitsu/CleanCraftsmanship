pub fn sort() -> bool {
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let result = sort();
        assert_eq!(result, true);
    }
}
