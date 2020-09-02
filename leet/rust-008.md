# leetcode 

## 8. 字符串转换整数 (atoi)




题目描述
评论 (723)
题解New
提交记录
请你来实现一个 atoi 函数，使其能将字符串转换成整数。

首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。

当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。

该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。

注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。

在任何情况下，若函数不能进行有效的转换时，请返回 0。

说明：

假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。如果数值超过这个范围，qing返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。

示例 1:

输入: "42"
输出: 42
示例 2:

输入: "   -42"
输出: -42
解释: 第一个非空白字符为 '-', 它是一个负号。
     我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
示例 3:

输入: "4193 with words"
输出: 4193
解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
示例 4:

输入: "words and 987"
输出: 0
解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
     因此无法执行有效的转换。
示例 5:

输入: "-91283472332"
输出: -2147483648
解释: 数字 "-91283472332" 超过 32 位有符号整数范围。 
     因此返回 INT_MIN (−231) 。
在真实的面试中遇到过这道题？

###　执行用时为 4 ms
```rust
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let (i32_min, i32_max) = (-2_i64.pow(31), 2_i64.pow(31) - 1);
        let mut result: i64 = 0;
        let mut minus = false;
        // simple state machine
        let mut num_matched = false;
        for ch in str.chars().into_iter() {
            if !num_matched {
                match ch {
                    ' ' => {},
                    '0'...'9' => {
                        num_matched = true;
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                    },
                    '-' => { num_matched = true; minus = true; }
                    '+' => { num_matched = true; }
                    _ => return 0
                }
            } else {
                match ch {
                    '0'...'9' => {
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                        if result > i32_max { break }
                    },
                    _ => break
                }
            }
        }
        result = if minus { -result } else { result };
        if result > i32_max { return i32_max as i32; }
        if result < i32_min { return i32_min as i32; }
        return result as i32;
        
    }
}
```


### c执行用时为 0 ms 的范例
```c
    if(strlen(str)==0) return 0;
    
    
    
    long result=0;
    char*s=NULL;
    
    int i=0,j=0;
    int len=strlen(str)+1;
    s=(char*)malloc(len*sizeof(char));
    char flag=0;
    while(i<len-1)
    {
        if(!flag){
            if(str[i]==' ')
            {
                i++;
                continue;
            }
            if(str[i]!='+'&&str[i]!='-'&&str[i]<'0'&&str[i]>'9')
            {
                return 0;
            }
            /*if(str[i]=='+'||str[i]=='-'||(str[i]>'0'&&str[i]<'9'))
            {
                i++;
                continue; 
            }*/
            flag=1;
            s[j]=str[i];
            j++;
            i++;
            continue;
        }
        if(str[i]>='0'&&str[i]<='9'){
            s[j]=str[i];
            j++;
            i++;
            continue;
        }
        
        break;
        
        

        
    }
    s[j]='\0';
    if(j==0) return 0;
    if(s[0]=='+'||s[0]=='-')
    {
        flag=s[0];
        s++;
    }
    while(*s=='0')
        s++;
    if(strlen(s)>10&&flag=='-') return -2147483648;
    if(strlen(s)>10&&flag=='+') return 2147483647;
    
    sscanf(s,"%ld",&result);
    
    
    if(flag=='-') result= -result;
    if(result<-2147483648) return -2147483648;
    if(result>2147483647) return 2147483647;
    return result;
    /*
    sscanf(str,"%s%d",s,&result);
    if(strlen(s)>10) return 2147483647;
    if(*str<'0'&&*str>'9'&&(*str!='+'||*str!='-'))
        sscanf(str,"%*s%d%*s",&result);
    else
        sscanf(str,"%d%*s",&result);
    //printf("%d\n",result);
    
    if(result<=2147483647&&result>=-2147483648)
        return result;
    
    return 2147483647;*/
    

```

```c
int myAtoi(char * str){
    int64_t ret = 0;
    int sign = 1;
    char *p = str;
    /* ignore white spaces */
    while (*p == ' ') p++;

    if (*p == '-') {
        sign = -1;
        p++;
    }
    else if (*p == '+') {
        sign = 1;
        p++;
    }

    while (*p >= '0' && *p <= '9') {
        ret = ret * 10 + (*p - '0');
        if (ret - 1 > INT32_MAX) ret = (int64_t)INT32_MAX + 1;
        //printf("%ld\n", ret);
        p++;
    }

    if (sign == -1) ret = -ret;
    if (ret > INT32_MAX) ret = INT32_MAX;
    if (ret < INT32_MIN) ret = INT32_MIN;
    
    return (int)ret;
}
```
## golang 执行用时为 4 ms 的范例
```go
执行用时为 4 ms 的范例
func myAtoi(str string) int {
    runes := []rune(str)
    i := 0
    for ; i < len(str); i++ {
        if string(str[i]) == " " {
            continue
        }
        break
    }
    
    if i >= len(str) {
        return 0
    }
    
    flag := true
    if string(str[i]) == "+" || string(str[i]) == "-"{
        flag = string(str[i]) == "+"
        i++
    }
    
    res := 0
    for ;i < len(str); i++ {
        if runes[i] >= rune('0') && runes[i] <= rune('9') {
            res = res * 10 + int(runes[i]) - int(rune('0'))
            if flag {
                if res > math.MaxInt32 {
                    return math.MaxInt32
                }
            } else {
                if -res < math.MinInt32 {
                    return math.MinInt32
                }
            }
        } else {
            break
        }
    }
    
    if !flag {
        return -res
    }
    
    return res
}
```
---
记录一下