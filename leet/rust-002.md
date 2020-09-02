# leetcode 

## 001. 两数之和




题目描述
评论 (2.0k)
题解New
提交记录
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
在真实的面试中遇到过这道题？

 

示例 1:

输入: 1
输出: "1"
示例 2:

输入: 4
输出: "1211"



###　执行用时为 4 ms 的范例
```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
    let mut index = 0;
    for num in &nums{
        map.insert(num,index);
        index+=1;
    }
    let mut index =0;
    for num in &nums{
        let need = target-num;
        if let Some(indexNeed) = map.get(&need) {
            if *indexNeed!=index{
                return vec![index,*indexNeed]
            }
        }
        index+=1;
    }
    panic!("no such thing")
    }
}
```

### 执行用时为 0 ms 的范例
```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let num2 = target - num;
            if index.contains_key(&num2) {
                return vec![i as i32, index[&num2]]
            }
            index.insert(num, i as i32);
        }
        unreachable!()
    }
}
```

### 执行用时为 40 ms 的范例
```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut rev = vec![-1i32;2];
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    rev[0] = i as i32;
                    rev[1] = j as i32;
                    return rev;
                }
            }
        }
        rev
    }
}
```

```go
func twoSum(nums []int, target int) []int {
    m := make(map[int]int)
	for i, n := range nums {
		if j, ok := m[n]; ok {
			return []int{j, i}
		}
		m[target-n] = i
	}
	return []int{}
}
```

```c
/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target) {
    
        static int a[2]={0}; 

	for (int i = 0; i < numsSize - 1; i++)
	{
		for (int j = i+1; j < numsSize; j++)
		{
			if (nums[i] + nums[j] == target)
			{
				a[0] = i;
				a[1] = j;
				return a;
			}
		}
	}
	return 0;
}
```

```c
执行用时为 4 ms 的范例
/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

//键值对应
struct node{
    int val;
    int index;
};

//比较大小
int comp(const void *a, const void *b)
{
    return ((struct node*)a)->val > ((struct node *)b)->val ? 1 : -1;
}

int* twoSum(int* nums, int numsSize, int target) {
    int *result = NULL;
    struct node* nodes = (struct node *)malloc(numsSize * sizeof(struct node));
    int i = 0;
    int begin = 0;
    int end = numsSize - 1;
    
    //将元素与标号对应起来
    for  (i = 0; i < numsSize; i ++)
    {
        nodes[i].val = nums[i];
        nodes[i].index = i;
    }
    
    qsort(nodes, numsSize, sizeof(struct node), comp);
    
    while (begin < end)
    {
        if (nodes[begin].val + nodes[end].val == target)
        {
            result = (int *)malloc(sizeof(int) * 2);
            result[0] = nodes[begin].index;
            result[1] = nodes[end].index;
            free(nodes);
            return result;
        }
        else if (nodes[begin].val + nodes[end].val > target)
            end --;
        else
            begin ++;
    }
    free(nodes);
    return result;
}
```
---
记录一下