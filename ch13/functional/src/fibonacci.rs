pub fn fibonacci(n: usize) -> Vec<u64> {
    let mut results: Vec<u64> = vec![1, 1];
    results.resize(n, 0);

    for i in 2..n {
        results[i] = results[i - 1] + results[i - 2];
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let target = vec![1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(target, fibonacci(target.len()));
    }
}
