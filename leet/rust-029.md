# leetcode 

## 29. 两数相除




题目描述
评论 (178)
题解(6)New
提交记录
给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。

返回被除数 dividend 除以除数 divisor 得到的商。

示例 1:

输入: dividend = 10, divisor = 3
输出: 3
示例 2:

输入: dividend = 7, divisor = -3
输出: -2
说明:

被除数和除数均为 32 位有符号整数。
除数不为 0。
假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231,  231 − 1]。本题中，如果除法结果溢出，则返回 231 − 1。

###　执行用时为 12 ms
```rust
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend=dividend;
        let mut divisor=divisor;
        let sign = if (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0) {
            -1
        } else {
            1
        };

        let mut adj = 0;
        let mut q = 0;
        let mut r = 0;
        
        if divisor == std::i32::MIN {
            if divisor == dividend {
                return 1
            } else {
                return 0
            }
        }

        if dividend == std::i32::MIN {
            if divisor == 1 {
                return std::i32::MIN;
            }
            if divisor == -1 {
                return std::i32::MAX;
            }
            dividend += divisor.abs();
            adj += 1;
        }
        
        dividend = dividend.abs();
        divisor = divisor.abs();
                       
        for i in (0..32).rev() {
            r <<= 1;
            if dividend & (1 << i) != 0 {
                r |= 1;
            }
            if r >= divisor {
                r -= divisor;
                q = q | (1 << i);
            }
        }
        
        if sign < 0 {
            -q - adj
        } else {
            q + adj
        }
    }
}
```

### 执行用时为 4 ms 的范例
```rust
const MIN_VALUE: i64 = i32::min_value() as i64;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut result = 0;
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        let negative = (dividend ^ divisor) < 0;

        // 防溢出处理
        if dividend == MIN_VALUE && divisor == -1  {
            return i32::max_value();
        }

        dividend = dividend.abs();
        divisor = divisor.abs();

        for _i in 0..=31 {
            let i = 31 - _i;

            // 如果dividend / 2^i大于divider，
            // 那么说明dividend减去2^i个divider以后还可以被divider整除
            // 那么重复上面的过程并把得到的result累加即可
            // 直到最后divided本身就小于divisor以后，说明此时的dividend即为余数
            if (dividend >> i) >= divisor {
                result += 1 << i;
                dividend -= divisor << i;
            }
        }

        if negative {
            -result as i32
        } else {
            result as i32
        }
    }
}

```

### It did not check out.
```c

```

### 执行用时为 12 ms 的范例
```c

```

### c执行用时为 4 ms 的范例
```c

```


### c执行用时为 8 ms 的范例
```c

```

## golang 执行用时为 4 ms 的范例
```go

```

## golang 执行用时为 8 ms 的范例
```go

```
---
记录一下