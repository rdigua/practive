# leetcode 

## 7. 整数反转




题目描述
评论 (1.2k)
题解New
提交记录
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

示例 1:

输入: 123
输出: 321
 示例 2:

输入: -123
输出: -321
示例 3:

输入: 120
输出: 21
注意:

假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

###　执行用时为 0 ms
```rust
//执行用时为 0 ms 的范例
impl Solution {
 pub fn reverse(x: i32) -> i32 {
        // const threshold : std::i32::MAX
        // const threshold:  std::i32::MAX;

        const MAX: i32 = std::i32::MAX / 10;
        const MIN: i32 = std::i32::MIN / 10;

        // println!("{} {}", MAX, MIN);

        let mut x = x;
        let mut ret = 0;
        while x != 0 {
            // println!("{}", ret);
            if ret > MAX || ret < MIN {
                return 0;
            }
            ret = ret * 10;
            let rem = x % 10;

            // println!("{} {}", ret, rem);

            if rem > 0 && ret > std::i32::MAX - rem || rem < 0 && ret < std::i32::MIN - rem {
                return 0;
            }
            ret = ret + rem;

            x = x / 10;
        }

        ret
    }
}
```

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input: i64 = x as i64;
        let mut result: i64 = 0;
        let mut digit: i64 = 0;
        let base: i64 = 2;
        let upper_bound: i64 = base.pow(31) - 1;
        let lower_bound: i64 = - base.pow(31);
        while input != 0 {
            digit = input % 10;
            result = result * 10 + digit;
            input = input / 10;
        }
        if result > upper_bound || result < lower_bound {
            return 0;
        }
        result as i32        
    }
}
```



### c执行用时为 0 ms 的范例
```c
int reverse(int x){
    int y = 0;
    while (x != 0) {
        int n = x % 10;
        // Checking the over/underflow.
        // Actually, it should be y>(INT_MAX-n)/10, but n/10 is 0, so omit it.
        if (y > INT_MAX / 10 || y < INT_MIN / 10) {
            return 0;
        }
        y = y * 10 + n;
        x /= 10;
    }
    return y;
}

```

## golang 执行用时为 0 ms 的范例
```go
const INT_MAX = int(^uint32(0) >> 1)
const INT_MIN = ^INT_MAX

func reverse(x int) int {
    if x == 0 {
		return 0
	}
	sign := 1
	if x < 0 {
		sign = -1
		x = x * -1
	}
	data := make([]int, 0, 16)
	for {
		if x < 10 {
			data = append(data, x)
			break
		}
		data = append(data, x%10)
		x /= 10
	}

	ret := 0
	for i := 0; i < len(data); i++ {
		ret = ret*10 + data[i]
		if ret > INT_MAX {
			return 0
		}
	}
	//fmt.Println(sign*ret, fmt.Sprintf("%+v", data))
	return sign * ret
}
```
---
记录一下