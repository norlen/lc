// [283. Move Zeroes](https://leetcode.com/problems/move-zeroes/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int non_zero = 0;
        for (int i = 0; i < nums.size(); ++i) {
            if (nums[i] != 0) {
                swap(nums[i], nums[non_zero]);
                non_zero++;
            }
        }
    }
};
