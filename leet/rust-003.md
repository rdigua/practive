# leetcode 

## 3. 无重复字符的最长子串




题目描述
评论 (1.1k)
题解New
提交记录
给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。

示例 1:

输入: "abcabcbb"
输出: 3 
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

###　执行用时为 0 ms
```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break
                }
            }
            let curr = end - start + 1;
            if curr > max { max = curr }
            end += 1
        }
        max as i32
    }
}
```

### 执行用时为 0 ms 的范例
```rust

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut max = 0;
        let mut len;

        for c in s.chars() {
            if let Some(v) = &s[i..j].find(c) {
                // println!("find {} at {}", c, v);
                i = i + v + 1;
            }

            len = j - i + 1;
            if max < len {
                max = len;
            }

            // println!("{} {} {}", i, j, len);
            j += 1;
        }

        // dvdf
        // abcabcbb
        max as i32
    }
}
```

### 执行用时为 4 ms 的范例
```rust


```

---
记录一下