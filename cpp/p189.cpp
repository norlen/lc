// [189. Rotate Array](https://leetcode.com/problems/rotate-array/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        // vector<int> b(nums.size());
        // for (int i = 0; i < nums.size(); ++i) {
        //     b[(i+k)%nums.size()] = nums[i];
        // }
        // nums = b;
        
        k %= nums.size();
        reverse(nums.begin(), nums.end());
        for (int i = 0; i < k/2; ++i) {
            swap(nums[i], nums[k-i-1]);
        }
        for (int i = 0; i < (nums.size()-k)/2; ++i) {
            swap(nums[k+i], nums[nums.size()-i-1]);
        }
    }
};
