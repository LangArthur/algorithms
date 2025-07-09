// A prefix-sum is a technique for efficiently calculating the sum of subarrays in an integer array
// ex:
// sum of subarray [i, j] = subarray[j] - subarray[i]
fn prefix_sums(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut prefix = vec![0; n + 1];
    for i in 1..n + 1 {
        prefix[i] = prefix[i - 1] + arr[i - 1]
    }
    prefix
}

fn main() {
    let arr = vec![2, 3, 1, 4];
    println!("{:?}", prefix_sums(&arr));
}
