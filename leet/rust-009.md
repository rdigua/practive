# leetcode 

## 9. 回文数




题目描述
评论 (927)
题解New
提交记录
判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

示例 1:

输入: 121
输出: true
示例 2:

输入: -121
输出: false
解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
示例 3:

输入: 10
输出: false
解释: 从右向左读, 为 01 。因此它不是一个回文数。
进阶:

你能不将整数转为字符串来解决这个问题吗？

###　执行用时为 8 ms
```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
       if x < 0 { return false }
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input % 10);
            input = input / 10;
        }
        let len = digits.len();
        // handle one digit
        if len < 2 { return true }
        // handle end with 0
        if digits[0] == 0 { return false }
        let mut i = 0;
        while i < len / 2 {
            if digits[i] != digits[len - 1 - i] {
                return false
            }
            i += 1;
        }
        true        
    }
}
```

### 执行用时为 36 ms 的范例
```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_copy = x;
        let mut reverse = 0;//反转的数
        while x_copy > 0 {
            reverse = reverse * 10 + x_copy % 10;
            x_copy = x_copy / 10;
        }

        return x == reverse;
    }
}
```

### c执行用时为 0 ms 的范例
```c
bool isPalindrome(int x){
    int i, len = 0;
    bool ok = true;
    int a[15];
    if(x < 0)
    {
        return false;
    }
    while(x)
    {
        i = x % 10;
        a[len++] = i;
        x /= 10;
    }
    for(i = 0; i < len / 2 && ok; ++i)
    {
        if(a[i] != a[len - i - 1])
        {
            ok = false;
        }
    }
    return ok;
}
```

### c执行用时为 20 ms 的范例
```c
bool isPalindrome(int x){
     if (x < 0 || (x % 10 == 0 && x != 0)) return false;
          return reverse(x) == x;
      }
      int reverse(int x) {
          int res = 0;
          while (x != 0) {
             if (res > INT_MAX / 10) return -1;
             res = res * 10 + x % 10;
             x /= 10;
         }
         return res;
}


```
## golang 执行用时为 12 ms 的范例
```go
/*
 * @lc app=leetcode.cn id=9 lang=golang
 *
 * [9] 回文数
 */

func isPalindrome(x int) bool {

	if x < 0 {
		return false
	}
	if x/10 == 0 {
		return true
	}
	oldx := x
	var orx int
	for x > 0 {
		a := x % 10

		orx = orx*10 + a

		x = x / 10

	}
	return orx == oldx
}
```
---
记录一下