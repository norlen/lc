// [260. Single Number III](https://leetcode.com/problems/single-number-iii/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        uint32_t bitmask = 0;
        for (int num : nums) {
            bitmask ^= num;
        }
        
        uint32_t rightmost_bit = bitmask & (-bitmask);
        
        uint32_t x = 0;
        for (int num : nums) {
            if (num & rightmost_bit) {
                x ^= num;
            }
        }
        
        return {static_cast<int>(x), static_cast<int>(bitmask^x)};
    }
};
