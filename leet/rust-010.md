# leetcode 

## 10. 正则表达式匹配

给定一个字符串 (s) 和一个字符模式 (p)。实现支持 '.' 和 '*' 的正则表达式匹配。

'.' 匹配任意单个字符。
'*' 匹配零个或多个前面的元素。
匹配应该覆盖整个字符串 (s) ，而不是部分字符串。

说明:

s 可能为空，且只包含从 a-z 的小写字母。
p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
示例 1:

输入:
s = "aa"
p = "a"
输出: false
解释: "a" 无法匹配 "aa" 整个字符串。
示例 2:

输入:
s = "aa"
p = "a*"
输出: true
解释: '*' 代表可匹配零个或多个前面的元素, 即可以匹配 'a' 。因此, 重复 'a' 一次, 字符串可变为 "aa"。
示例 3:

输入:
s = "ab"
p = ".*"
输出: true
解释: ".*" 表示可匹配零个或多个('*')任意字符('.')。
示例 4:

输入:
s = "aab"
p = "c*a*b"
输出: true
解释: 'c' 可以不被重复, 'a' 可以被重复一次。因此可以匹配字符串 "aab"。
示例 5:

输入:
s = "mississippi"
p = "mis*is*p*."
输出: false
在真实的面试中遇到过这道题？

###　执行用时为 0 ms
```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        
        
        if p.is_empty() { return s.is_empty() }
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_v2(&s, &p)
    }
}

fn is_match_v2(s: &[char], p: &[char]) -> bool {

    let mut states = vec![vec![false; p.len() + 1]; s.len() + 1];
    states[s.len()][p.len()] = true;

    for i in (0..=s.len()).rev() {
        for j in (0..=(p.len() - 1)).rev() {

            let first_match = i < s.len() && (p[j] == s[i] || p[j] == '.');
            
            states[i][j] = if j + 1 < p.len() && p[j + 1] == '*' {
                states[i][j + 2] || (first_match && states[i + 1][j])
            } else {
                first_match && states[i + 1][j + 1]
            };
        }
    }

    states[0][0]
}
```

### 执行用时为 0 ms 的范例
```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut matched = vec![vec![false; p.len() + 1]; 2];
        let (mut now, mut past) = (0, 1);
        for i in (0..s.len() + 1).rev() {
            for j in (0..p.len() + 1).rev() {
                let (s, p) = (&s[i..], &p[j..]);
                matched[now][j] = if p.is_empty() {
                    s.is_empty()
                } else {
                    let first_match = !s.is_empty()
                        && (s.as_bytes()[0] == p.as_bytes()[0]
                            || p.as_bytes()[0] == b'.');
                    if p.len() >= 2 && p.as_bytes()[1] == b'*' {
                        (first_match && matched[past][j]) || matched[now][j + 2]
                    } else {
                        first_match && matched[past][j + 1]
                    }
                };
            }
            std::mem::swap(&mut now, &mut past);
        }
        matched[past][0]
    }
}
```

### c执行用时为 32 ms 的范例
```c
bool isMatch(char * s, char * p){
    if (*p == '\0') {
        return *s == '\0';
    }
    /* p's length 1 is special case */
    if (*(p + 1) == '\0' || *(p + 1) != '*') {
        if (*s == '\0' || ( *p != '.' && *s != *p)) {
            return false;
        } else {
            return isMatch(s + 1, p + 1);
        }
    }
    int len = strlen(s);
    int i = -1;
    while (i < len && (i < 0 || *p == '.' || *p == *(s + i))) {
        if (isMatch(s + i + 1, p + 2)) {
            return true;
        }
        i++;
    }
    return false;
}

```

### c执行用时为 8 ms 的范例
```c
执行用时为  ms 的范例

int all(char *s ,int start, int end, char a)
{
    int i;
    if(a == '.')
    {
        return 1;
    }
    for(i = start; i < end; i++)
    {
        if(*(s+i) != a)
        {
            return 0;
        }
    }
    return 1;
}

bool isMatch(char * s, char * p){
    
    char *m;
    int slen, plen, i, j, k;
    int result;

    slen = strlen(s) + 1;
    plen = strlen(p) + 1;
    m = (char *)malloc(slen * plen);
    memset(m, 0, slen*plen);
    *m = 1;
    for(i = 1; i <= strlen(p); i++)
    {
        if(i % 2)
        {
            *(m + i*slen) = 0;
            continue;
        }
        for(j = 1; j < i; j += 2)
        {
            if(*(p + j) != '*')
            {
                *(m + i*slen) = 0;
                break;
            }
        }
        if(j >= i)
        {
            *(m + i*slen) = 1;
        }
    }
    for(i = 1; i < plen; i++)
    {
        for(j = 1; j < slen; j++)
        {
            if(*(p + i - 1) != '.' && *(p + i - 1) != '*')
            {
                if(*(m + (i - 1)*slen + (j - 1)) && *(s + j - 1) == *(p + i - 1))
                {
                    *(m + i * slen + j) = 1;
                }
                else
                {
                    *(m + i * slen + j) = 0;
                }
            }
            else if(*(p + i - 1) == '.')
            {
                if(*(m + (i - 1)*slen + (j - 1)))
                {
                    *(m + i * slen + j) = 1;
                }
                else
                {
                    *(m + i * slen + j) = 0;
                }
            }
            else
            {
                for(k = 0; k <= j; k++)
                {
                    //printf("i = %d, j = %d\n",i,j);
                    if(*(m + (i - 2) * slen + k) && all(s, k, j, *(p + i - 1 - 1)))
                    {
                        *(m + i * slen + j) = 1;
                        break;
                    }
                    if(k == j)
                    {
                       *(m + i * slen + j) = 0;
                    }
                }
            }
        }
    }
    
    result = *(m + plen * slen - 1);
    free(m);
    return result;
}
```
## golang 执行用时为 0 ms 的范例
```go
func isMatch(s string, p string) bool {
    lena, lenb := len(s), len(p)
    dp := make([][]bool, lena + 1)
    for i := 0; i < lena + 1; i++ {
        dp[i] = make([]bool, lenb + 1)
    }
    dp[0][0] = true
    for i := 0; i <= lena; i++ {
        for j := 1; j <= lenb; j++ {
            if i > 0 && s[i - 1] == p[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]
            }
            if i > 0 && p[j - 1] == '.' {
                dp[i][j] = dp[i - 1][j - 1]
            }
            if p[j - 1] == '*' {
                if p[j - 2] == '.' {
                    for k := 0; k <= i; k++ {
                        if dp[k][j - 2] {
                            dp[i][j] = dp[k][j - 2]
                        }
                    }
                } else {
                    if dp[i][j - 2] {
                        dp[i][j] = dp[i][j - 2]
                    }
                    for k := 1; k <= i; k++ {
                        if p[j - 2] != s[i - 1 + 1 - k] {
                            break
                        }
                        if dp[i - k][j - 2] {
                            dp[i][j] = dp[i - k][j - 2]
                        }
                
                    }
                }
            }
        }
    }
    return dp[lena][lenb]
}
```
---
记录一下