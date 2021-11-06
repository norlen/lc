// [376. Wiggle Subsequence](https://leetcode.com/problems/wiggle-subsequence/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        if (nums.size() == 0) return 0;
        int up = 1, down = 1;
        
        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i-1] < nums[i]) {
                up = down + 1;
            } else if (nums[i-1] > nums[i]) {
                down = up + 1;
            }
        }
        return max(up, down);
    }
};