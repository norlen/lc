// [977. Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        vector<int> m(nums.size());
        
        int j = 0;
        int k = nums.size() - 1;
        for (int i = nums.size()-1; i >= 0; --i) {
            if (k < 0 || abs(nums[j]) >= abs(nums[k])) {
                m[i] = nums[j]*nums[j];
                ++j;
            } else {
                m[i] = nums[k]*nums[k];
                --k;
            }
        }
        return m;
    }
};
