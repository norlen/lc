// [198. House Robber](https://leetcode.com/problems/house-robber/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int rob(vector<int>& nums) {
        if (nums.size() == 1) {
            return nums[0];
        }
        int a = 0;
        int b = nums[0];
        
        for (int i = 1; i < nums.size(); ++i) {
            int c = max(b, a + nums[i]);
            a = b;
            b = c;
        }
        return max(a, b);
    }
};
