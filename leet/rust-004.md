# leetcode 

## 4. 寻找两个有序数组的中位数




题目描述
评论 (807)
题解New
提交记录
给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。

请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。

你可以假设 nums1 和 nums2 不会同时为空。

示例 1:

nums1 = [1, 3]
nums2 = [2]

则中位数是 2.0
示例 2:

nums1 = [1, 2]
nums2 = [3, 4]

则中位数是 (2 + 3)/2 = 2.5

###　执行用时为 12 ms
```rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
       let len1 = nums1.len();
        let len2 = nums2.len();
        let mut nums_1 = nums1.clone();
        let mut nums_2 = nums2.clone();
        if len1 == 0 {
            if len2 == 1 {
                return nums2[0] as f64;
            } else {
                if len2 % 2 == 0 {
                    let v = len2 / 2;
                    return ((nums2[v] + nums2[v-1]) as f64)/2.0;
                } else {
                    let v = len2 / 2;
                    return nums2[v] as f64;
                }
            }
        }
        if len2 == 0 {
            if len1 == 1 {
                return nums1[0] as f64;
            } else {
                if len1 % 2 == 0 {
                    let v = len1 / 2;
                    return ((nums1[v] + nums1[v-1]) as f64)/2.0;
                } else {
                    let v = len1 / 2;
                    return nums1[v] as f64;
                }
            }
        }
        let mut limit = 0;
        let mut times = 0;
        let mut  flag = 0;
        let len_sum = len1 + len2;
        if len_sum % 2 == 0 {
            flag = 2;
//            循环的次数
            limit = len_sum / 2 + 1
        } else {
            flag = 1;
            limit = len_sum / 2 + 1
        }
        let mut ans = vec![];
        while times < limit {
            let mut l1 = nums_1.len();
            let mut l2 = nums_2.len();
            let l = ans.len();
            if l1 != 0 && l2 != 0 {
                if nums_1[0] > nums_2[0] {
                    let el = nums_2[0];
                    ans.push(el);
                    nums_2.remove(0 as usize);
                } else {
                    let el = nums_1[0];
                    ans.push(el);
                    nums_1.remove(0 as usize);
                }
            } else if l1 == 0 && l2 != 0 {
                let el = nums_2[0];
                ans.push(el);
                nums_2.remove(0 as usize);
            } else if l2 == 0 && l1 != 0 {
                let el = nums_1[0];
                ans.push(el);
                nums_1.remove(0 as usize);
            }
            times += 1
        }
        let l = ans.len();
        if flag == 2 {
           return  ((ans[l-1] + ans[l-2]) as f64)/2.0;
        } else {
            return ans[l-1] as f64;
        }
    }
}
```

### 执行用时为 4 ms 的范例
```rust
use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let (a, b, m, n) = if m > n {
            (nums2, nums1, n, m)
        } else {
            (nums1, nums2, m, n)
        };

        // 0 .. m
        let mut left = 0;
        let mut right = m;

        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            if i < right && b[j - 1] > a[i] {
                left = i + 1;
            } else if i > left && a[i - 1] > b[j] {
                right = i - 1;
            } else {
                let max_left = if i == 0 {
                    b[j - 1]
                } else if j == 0 {
                    a[i - 1]
                } else {
                    cmp::max(a[i - 1], b[j - 1])
                };

                if (m + n) % 2 == 1 {
                    return max_left as f64;
                }

                let min_right = if i == m {
                    b[j]
                } else if j == n {
                    a[i]
                } else {
                    cmp::min(b[j], a[i])
                };

                return (max_left + min_right) as f64 / 2.0;
            }
        }
        return 0.0;
    }
}
```

### 执行用时为 8 ms 的范例
```rust
执行用时为 8 ms 的范例
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let  (m, n) = (nums1.len(), nums2.len());
        let mid_idx = (m + n - 1) / 2;
        let mut cur_idx = 0;
        
        let (mut i, mut j) = (0, 0);
        let (mut ans, mut cur_num) = (0.0, 0);
        while i < m || j < n {
            if j == n || (i < m && nums1[i] <= nums2[j]) { 
                cur_num = nums1[i]; 
                i+=1;
            }else { 
                cur_num = nums2[j]; 
                j+=1;
            };

            if cur_idx == mid_idx {
                if (m + n) % 2 == 1 {
                    return cur_num as f64;
                } else {
                    ans += cur_num as f64;
                }
            }
            if cur_idx-1 == mid_idx {
                if (m + n) % 2 == 0 {
                    ans += cur_num as f64;
                    return ans/2.0;
                }
            }
            cur_idx += 1;
        }
        ans
    }
}

```

---
记录一下