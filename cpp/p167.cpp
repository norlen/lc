// [167. Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& numbers, int target) {
        int l = 0;
        int r = numbers.size() - 1;
        
        while (l < r) {
            int val = numbers[l] + numbers[r];
            if (val == target) {
                return {l+1, r+1};
            } else if (val < target) {
                l++;
            } else {
                r--;
            }
        }
        return {-1, -1};
    }
};
