impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a: Vec<(i32, usize)> = nums.iter().copied().enumerate().map(|(i, num)| (num, i)).collect();

        a.sort();

        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            let cur = a[i].0 + a[j].0;
            if cur == target {
                return vec![
                    a[i].1.min(a[j].1) as i32,
                    a[i].1.max(a[j].1) as i32,
                ];
            } else if cur < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![]
    }
}
