# leetcode 

## 6. Z 字形变换




题目描述
评论 (384)
题解New
提交记录
将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：

L   C   I   R
E T O E S I I G
E   D   H   N
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);
示例 1:

输入: s = "LEETCODEISHIRING", numRows = 3
输出: "LCIRETOESIIGEDHN"
示例 2:

输入: s = "LEETCODEISHIRING", numRows = 4
输出: "LDREOEIIECIHNTSG"
解释:

L     D     R
E   O E   I I
E C   I H   N
T     S     G
在真实的面试中遇到过这道题？

###　执行用时为 4 ms
```rust
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let s = s.as_bytes();
        let mut ret = String::with_capacity(s.len());
        for n in 0..num_rows {
            let mut idx = 0;
            for &i in [n].iter().chain([(num_rows - n - 1) * 2, 2 * n].iter().cycle()) {
                if idx + i != idx || idx == 0 {
                    idx += i;
                    if idx as usize >= s.len() {
                        break;
                    }
                    ret.push(s[idx as usize] as char);
                }
            }
        }
        ret
    }
}
```

### golang执行用时为 4 ms 的范例
```go
func convert(s string, numRows int) string {
    if numRows <= 0 {
		return ""
	}
	if numRows == 1 {
		return s
	}
	ret := []byte{}
	for i := 0; i < numRows; i++ {
		for j, f := i, true; j < len(s); {
			ret = append(ret, s[j])
			if i == 0 || i == numRows-1 {
				j = j + 2*(numRows-1)
				continue
			}
			if f {
				j = j + 2*(numRows-1-i)
			} else {
				j = j + 2*i
			}
			f = !f
		}
	}
	return string(ret)
}

```

### c执行用时为 4 ms 的范例
```c
char* convert(char* s, int numRows) {
	int times = 0, num = 0, j = 0;
	static char recha[10000];
	int lenOfStr = strlen(s);
	if (numRows == 1)
		return s;
	if (numRows == 2)
	{
		while (2 * num < lenOfStr)
		{
			recha[j] = *(s + 2 * num);
			num++;
			j++;
		}
		num = 0;
		while (2 * num + 1 < lenOfStr)
		{
			recha[j] = *(s + 2 * num + 1);
			num++;
			j++;
		}
	}
	else {
		int numper = 2 * (numRows - 1);
		int k;
		for (num = 0; num <= numRows - 1; num++)
		{
			if (num == 0 || num == numRows - 1)
			{
				k = 0;
				while (numper * k + num < lenOfStr)
				{
					recha[j] = s[numper * k + num];
					j++;
					k++;
				}
			}
			else
			{
				k = 0;
				while (numper * k + num < lenOfStr)
				{

					recha[j] = s[numper * k + num];
					j++;
					if (numper * k + (numper - num) < lenOfStr)
					{
						recha[j] = s[numper * k + (numper - num)];
						j++;
					}
					k++;
				}
			}
		}
	}
	recha[j] = '\0';
	return recha;
}

```

## golang 执行用时为 12 ms 的范例
```go
func convert(s string, numRows int) string {
    if numRows == 1 {
		return s
	}

	var cs []uint8
	var count int
	var index int
	for i := 0; i < numRows; i++{
		count = 0
		index = count*(2*numRows-2) + i
		for index < len(s){
			cs = append(cs, s[index])
			if i != 0 && i != numRows - 1 {
				index = (count + 1) * (2 * numRows - 2) - i
				if index >= len(s) {
					break
				}
				cs = append(cs, s[index])
			}
			count++
			index = count*(2*numRows-2) + i
		}
	}

	return string(cs)
}
```
---
记录一下