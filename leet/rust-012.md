# leetcode 

## 12. 整数转罗马数字
 
题目描述
评论 (310)
题解(10)New
提交记录
罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。

字符          数值
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：

I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
给定一个整数，将其转为罗马数字。输入确保在 1 到 3999 的范围内。

示例 1:

输入: 3
输出: "III"
示例 2:

输入: 4
输出: "IV"
示例 3:

输入: 9
输出: "IX"
示例 4:

输入: 58
输出: "LVIII"
解释: L = 50, V = 5, III = 3.
示例 5:

输入: 1994
输出: "MCMXCIV"
解释: M = 1000, CM = 900, XC = 90, IV = 4.

###　执行用时为 12 ms
```rust
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let reps = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut res = String::new();

        for (i, &v) in values.iter().enumerate() {
            if num >= v {
                let count = num / v;
                for j in 0..count {
                    res.push_str(reps[i]);
                    num -= v;
                }
            }
        }

        res
    }
}

```

### 执行用时为 12 ms 的范例
```rust
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
              let table: Vec<(i32, &'static str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I")
        ];

        let mut num = num;
        let mut sb = String::new();
        for p in table.iter() {
            if num >= p.0 {
                for _ in 0..(num / p.0) {
                    sb.push_str(p.1);
                }
                num = num % p.0
            }
        }
        sb  
    }
}
```

### It did not check out.
```c
=================================================================
==30==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x60200000413a at pc 0x00000040294e bp 0x7ffea41f2100 sp 0x7ffea41f20f8
READ of size 1 at 0x60200000413a thread T0
    #3 0x7ff3fc21c2e0 in __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x202e0)

0x60200000413a is located 0 bytes to the right of 10-byte region [0x602000004130,0x60200000413a)
allocated by thread T0 here:
    #0 0x7ff3fd6a6498 in __interceptor_calloc (/usr/local/lib64/libasan.so.5+0xe8498)
    #3 0x7ff3fc21c2e0 in __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x202e0)

Shadow bytes around the buggy address:
  0x0c047fff87d0: fa fa 00 05 fa fa 00 02 fa fa 00 07 fa fa 00 02
  0x0c047fff87e0: fa fa 00 02 fa fa 00 02 fa fa 00 02 fa fa 00 07
  0x0c047fff87f0: fa fa 00 02 fa fa 00 02 fa fa 00 02 fa fa 00 02
  0x0c047fff8800: fa fa 00 02 fa fa 00 07 fa fa 00 02 fa fa 00 02
  0x0c047fff8810: fa fa 00 02 fa fa 00 02 fa fa 00 02 fa fa 00 02
=>0x0c047fff8820: fa fa 00 02 fa fa 00[02]fa fa fa fa fa fa fa fa
  0x0c047fff8830: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8840: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8850: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8860: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8870: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==30==ABORTING


char * intToRoman(int num){
   
    char *s = (char *)calloc(10, sizeof(char));
        if (num<1) {
        return NULL;
        }
        if (num>3999) {
        return NULL;
        }
    
    int i = 0;
    while (num > 0) {
        while (num >= 1000) {
            s[i] = 'M';
            num -= 1000;
            i++;
        }
        if (num >= 900) {
            s[i] = 'C'; s[i + 1] = 'M';
            i += 2;
            num -= 900;
        }
        while (num >= 500) {
            s[i] = 'D';
            num -= 500;
            i++;
        }
        if (num >= 400) {
            s[i] = 'C'; s[i + 1] = 'D';
            i += 2;
            num -= 400;
        }
        while (num >= 100) {
            s[i] = 'C';
            num -= 100;
            i++;
        }
        if (num >= 90) {
            s[i] = 'X'; s[i + 1] = 'C';
            i += 2;
            num -= 90;
        }
        while (num >= 50) {
            s[i] = 'L';
            num -= 50;
            i++;
        }
        if (num >= 40) {
            s[i] = 'X'; s[i + 1] = 'L';
            i += 2;
            num -= 40;
        }
        while (num >= 10) {
            s[i] = 'X';
            num -= 10;
            i++;
        }
        if (num >= 9) {
            s[i] = 'I'; s[i + 1] = 'X';
            i += 2;
            num -= 9;
        }
        while (num >= 5) {
            s[i] = 'V';
            num -= 5;
            i++;
        }
        if (num >= 4) {
            s[i] = 'I'; s[i + 1] = 'V';
            i += 2;
            num -= 4;
        }
        while (num >= 1) {
            s[i] = 'I';
            num -= 1;
            i++;
        }
    }
    return s;
}


```

### 执行用时为 12 ms 的范例
```c
static void num2char(char **num, int bit, int n)
{
    int i;
    char low, mid, high;
    char *p = *num;

    switch (n) {
    case 2:
        low = 'C';
        mid = 'D';
        high = 'M';
    break;
    case 1:
        low = 'X';
        mid = 'L';
        high = 'C';
    break;
    case 0:
        low = 'I';
        mid = 'V';
        high = 'X';
    break;
    }

    if (bit > 0) {
        switch (bit) {
        case 1:
        case 2:
        case 3:
            for (i = 0; i < bit; i++) {
                *p++ = low;
            }
            break;
        case 4:
            *p++ = low;
            *p++ = mid;
            break;
        case 5:
            *p++ = mid;
            break;
        case 6:
        case 7:
        case 8:
            *p++ = mid;
            for (i = 5; i < bit; i++) {
                *p++ = low;
            }
            break;
        case 9:
            *p++ = low;
            *p++ = high;
            break;
        }
    }

    *num = p;
}

static char roman_numeral[64];

char * intToRoman(int num){
    char *p = &roman_numeral[0];
    int thousand_bit = num / 1000;
    int hundred_bit = (num % 1000) / 100;
    int ten_bit = (num % 100) / 10;
    int one_bit = num % 10;
    int i;

    memset(roman_numeral, 0, sizeof(roman_numeral));

    if (thousand_bit > 0) {
        if (thousand_bit < 4) {
            for (i = 0; i < thousand_bit; i++) {
                *p++ = 'M';
            }
        }
    }

    num2char(&p, hundred_bit, 2);
    num2char(&p, ten_bit, 1);
    num2char(&p, one_bit, 0);

    return roman_numeral;
}

```

### c执行用时为 4 ms 的范例
```c
char * intToRoman(int num)
{
    char* RomanChar=malloc(17);
    memset(RomanChar,0,17);
    int i=0;
    int j=0;
    while(num>0)
    {
        if(num>=1000)
        {
            for(j=0;j<num/1000;j++,i++)
            {
                RomanChar[i]='M';
            }
            num%=1000;
        }
        if(num>=500)
        {
            if(num/900==1)
            {
                RomanChar[i]='C';
                i++;
                RomanChar[i]='M';
                i++;
                num%=900;
            }
            else
            {
                RomanChar[i]='D';
                i++;
                num%=500;
            }
        }
        if(num>=100)
        {
            if(num/400==1)
            {
                RomanChar[i]='C';
                i++;
                RomanChar[i]='D';
                i++;
                num%=400;
            }
            else
            {
                for(j=0;j<num/100;j++)
                {
                    RomanChar[i]='C';
                    i++;
                }
                num%=100;
            }
        }
        if(num>=50)
        {
            if(num/90==1)
            {
                RomanChar[i]='X';
                i++;
                RomanChar[i]='C';
                i++;
                num%=90;
            }
            else
            {
                RomanChar[i]='L';
                i++;
                num%=50;
            }
        }
        if(num>=10)
        {
            if(num/40==1)
            {
                RomanChar[i]='X';
                i++;
                RomanChar[i]='L';
                i++;
                num%=40;
            }
            else
            {
                for(j=0;j<num/10;j++)
                {
                    RomanChar[i]='X';
                    i++;
                }
                num%=10;
            }
        }
        if(num>=5)
        {
            if(num/9==1)
            {
                RomanChar[i]='I';
                i++;
                RomanChar[i]='X';
                i++;
                num%=9;
            }
            else
            {
                RomanChar[i]='V';
                i++;
                num%=5;
            }
        }
        if(num>=1)
        {
            if(num/4==1)
            {
                RomanChar[i]='I';
                i++;
                RomanChar[i]='V';
                i++;
                num%=4;
            }
            else
            {
                for(j=0;j<num;j++)
                {
                    RomanChar[i]='I';
                    i++;
                }
                num=0;
            }
        }
    }
    RomanChar[i]='\0';
    return RomanChar;
}

```


### c执行用时为 8 ms 的范例
```c

char * intToRoman(int num){
    char *s = (char *)calloc(17, sizeof(char));
    int i = 0;
    while (num > 0) {
        while (num >= 1000) {
            s[i] = 'M';
            num -= 1000;
            i++;
        }
        if (num >= 900) {
            s[i] = 'C'; s[i + 1] = 'M';
            i += 2;
            num -= 900;
        }
        while (num >= 500) {
            s[i] = 'D';
            num -= 500;
            i++;
        }
        if (num >= 400) {
            s[i] = 'C'; s[i + 1] = 'D';
            i += 2;
            num -= 400;
        }
        while (num >= 100) {
            s[i] = 'C';
            num -= 100;
            i++;
        }
        if (num >= 90) {
            s[i] = 'X'; s[i + 1] = 'C';
            i += 2;
            num -= 90;
        }
        while (num >= 50) {
            s[i] = 'L';
            num -= 50;
            i++;
        }
        if (num >= 40) {
            s[i] = 'X'; s[i + 1] = 'L';
            i += 2;
            num -= 40;
        }
        while (num >= 10) {
            s[i] = 'X';
            num -= 10;
            i++;
        }
        if (num >= 9) {
            s[i] = 'I'; s[i + 1] = 'X';
            i += 2;
            num -= 9;
        }
        while (num >= 5) {
            s[i] = 'V';
            num -= 5;
            i++;
        }
        if (num >= 4) {
            s[i] = 'I'; s[i + 1] = 'V';
            i += 2;
            num -= 4;
        }
        while (num >= 1) {
            s[i] = 'I';
            num -= 1;
            i++;
        }
    }
    return s;
}
```

## golang 执行用时为 4 ms 的范例
```go
func intToRoman(num int) string {
    ret := ""
    for i:= 0; i < num/1000; i++ {
        ret += "M"
    }
    
    num %= 1000
    switch (num/100) {
    case 1:
        ret += "C"
    case 2 :
        ret += "CC"
    case 3:
        ret += "CCC"
    case 4:
        ret += "CD"
    case 5:
        ret += "D"
    case 6:
        ret += "DC"
    case 7:
        ret += "DCC"
    case 8:
        ret += "DCCC"
    case 9:
        ret += "CM"
    }
    
    num %= 100
    switch (num/10) {
    case 1:ret+="X"
    case 2:ret+="XX"
        case 3:ret+="XXX"
    case 4:ret+="XL"
    case 5:ret+="L"
    case 6:ret+="LX"
    case 7:ret+="LXX"
    case 8:ret+="LXXX"
        case 9:ret+="XC"
    }
    
    num %= 10
    switch (num) {
    case 1:ret+="I"
    case 2:ret+="II"
    case 3:ret+="III"
    case 4:ret+="IV"
    case 5:ret+="V"
    case 6:ret+="VI"
    case 7:ret+="VII"
    case 8:ret+="VIII"
        case 9:ret+="IX"
    }
    return ret
}
```

## golang 执行用时为 8 ms 的范例
```go
func intToRoman(num int) string {
     var values = []int{1,4,5,9,10,40,50,90,100,400,500,900, 1000}
    var str = []string{"I", "IV", "V", "IX", "X", "XL","L", "XC", "C", "CD", "D", "CM","M"}
    
    result := ""
    slen := len(values)
    index := slen-1
    for num > 0  && index >= 0{
        if num >= values[index] {
            result += str[index]
            num -= values[index]
        } else {
            index --
        }
    }
    return result 
}
}
```
---
记录一下