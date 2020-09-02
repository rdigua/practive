# leetcode 

## 28. 实现strStr()




题目描述
评论 (611)
题解(14)New
提交记录
实现 strStr() 函数。

给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如果不存在，则返回  -1。

示例 1:

输入: haystack = "hello", needle = "ll"
输出: 2
示例 2:

输入: haystack = "aaaaa", needle = "bba"
输出: -1
说明:

当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。

对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。

###　执行用时为 0 ms
```rust
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
            if needle.is_empty() { return 0 }
        haystack.find(&needle).map_or(-1_i32, |v| v as i32)    
    }
}
```

### 执行用时为 4 ms 的范例
```rust

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

## golang 执行用时为 0 ms 的范例
```go
func strStr(haystack string, needle string) int {
    if needle == "" {
        return 0
    }
    if len(haystack) < len(needle){
        return -1
    }
    for i := 0; i < len(haystack); i++ {
		if haystack[i] == needle[0] {
			for j := 0; j < len(needle); j++ {
                if j + i >= len(haystack){
                    return -1
                }
				if needle[j] != haystack[j+i] {
					break
				}	
                if j == len(needle) - 1{
					return i
				}
			}
		}
	}
    return -1
}
```

## golang 执行用时为 0 ms 的范例
```go
    if needle == "" || haystack == needle {
        return 0}
    if len(needle) > len(needle)  {
        return -1}
    for i:=0;;i++{
        for j:=0;;j++{
            if j == len(needle){
                return i
            }
            if i+j == len(haystack){
                return -1
            }
            if (haystack[i+j]!=needle[j]){
                break                
            }
        }
    } 
```
---
记录一下