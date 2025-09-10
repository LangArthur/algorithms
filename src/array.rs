// Used for efficient range queries or updates, especially in problems related to cumulative sums
// A prefix-sum is a technique for efficiently calculating the sum of subarrays in an integer array
// ex:
// sum of subarray [i, j] = prefix_sum[j] - prefix_sum[i]
pub fn prefix_sums<T>(arr: &[T]) -> Vec<T>
where
    T: std::ops::Add<Output = T> + Copy + Default,
{
    let n = arr.len();
    if n == 0 {
        return [].to_vec();
    }
    let mut prefix = vec![T::default(); n + 1];
    for i in 1..n + 1 {
        prefix[i] = prefix[i - 1] + arr[i - 1]
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!(
            Vec::<i32>::from([0, 1, 3, 8, 12]),
            prefix_sums(&[1, 2, 5, 4])
        );
        assert_eq!(
            Vec::<i32>::from([0, 0, 0, 0, 0]),
            prefix_sums(&[0, 0, 0, 0])
        );
        assert_eq!(
            Vec::<i32>::from([0, 1, 1, 1, 5]),
            prefix_sums(&[1, 0, 0, 4])
        );
        assert_eq!(Vec::<i32>::from([0, -3, -3, 4]), prefix_sums(&[-3, 0, 7]));
    }

    #[test]
    fn test_empty() {
        assert_eq!(Vec::<i32>::from([]), prefix_sums(&[]))
    }

    #[test]
    fn test_float() {
        assert_eq!(
            Vec::<f32>::from([0f32, 1f32, 3.3f32, 8.5f32, 12.5f32]),
            prefix_sums(&[1f32, 2.3f32, 5.2f32, 4f32])
        )
    }
}
