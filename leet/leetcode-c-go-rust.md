# leetcode

## 1. Two Sum    

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


```go
func twoSum(nums []int, target int) []int {
    hash := make(map[int]int, len(nums))
    for k, v := range nums {
        if j, ok := hash[target-v]; ok {
            return []int{j, k}
        }
        hash[v] = k
    }
    return nil
}
```
