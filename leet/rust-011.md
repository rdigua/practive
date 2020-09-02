# leetcode 

## 11. 盛最多水的容器

给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

说明：你不能倾斜容器，且 n 的值至少为 2。



图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

 

示例:

输入: [1,8,6,2,5,4,8,3,7]
输出: 49

###　执行用时为 0 ms
```rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let   (mut s,mut e) = (0_usize,height.len()-1);
        if e==1 {
            return cmp::min(height[0],height[1])
        }
        let mut max = 0i32;
        while e-s>=1 {
            if height[s] <= height[e] {
                
                if ((e-s) as i32)*height[s] > max { max =((e-s) as i32)*height[s] ;};
                s+=1;
            }else{
                
                if ((e-s) as i32)*height[e] > max { max =((e-s) as i32)*height[e] ;};
                e-=1;
            }
        }
        max
        
    }
    
}

```

### 执行用时为 4 ms 的范例
```rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        
        let mut result=std::i32::MIN;
        let mut i=0usize;
        let mut j=height.len()-1;
        while(i<j){
            let area=(j-i) as i32*std::cmp::min(height[i],height[j]);
            if area>result{
                result=area;
            }
            
            if height[i]<height[j]{
                i+=1;
            }else{
                j-=1;
            }
        }
//        for i in 0..height.len(){
//            for j in i+1..height.len(){
//                let area=(j-i) as i32*std::cmp::min(height[i],height[j]);
//                if area>result{
//                    result=area;
//                }
//            }
//        }
        result
    }
}
```

### c执行用时为 4 ms 的范例
```c
int maxArea(int* height, int heightSize) {
    int maxarea=0,left=0,right=heightSize-1;
    int area=0;
    while(left<right)
    {
        if(height[left]<height[right])
        {    area=height[left]*(right-left);
             left++;
             if(area>maxarea)
                 maxarea=area;
        }else
        {
            area=height[right]*(right-left);
            right--;
            if(area>maxarea)
                maxarea=area;
        }
    }
    return maxarea;
}
```

### c执行用时为 8 ms 的范例
```c
int maxArea(int* height, int heightSize)
{
    int min = 0, max = heightSize - 1;
    int area_max = 0;
    while (min < max) {
        int area = (max - min) * (height[min] < height[max] ? height[min] : height[max]);
        area_max = area > area_max ? area : area_max;
        if (height[min] < height[max]) {
            while (++min < max && height[min] <= height[min - 1]) {
                continue;
            }
        } else {
            while (min < --max && height[max] <= height[max + 1]) {
                continue;
            }
        }
    }
    return area_max;
}
```
## golang 执行用时为 12 ms 的范例
```go
func maxArea(height []int) int {
    res := 0
    i, j := 0, len(height)-1
    for i < j {
        res = max(res, (j-i) * min(height[i], height[j]))
        if height[i] < height[j] {
            i++
        } else {
            j--
        }
    }
    return res
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}
```
---
记录一下