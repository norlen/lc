// [1874. Minimize Product Sum of Two Arrays](https://leetcode.com/problems/minimize-product-sum-of-two-arrays/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int minProductSum(vector<int>& nums1, vector<int>& nums2) {
        // Multiply lowest with highest number.
        sort(nums1.begin(), nums1.end());
        sort(nums2.rbegin(), nums2.rend());

        int s = 0;
        for (int i = 0; i < nums1.size(); ++i) {
            s += nums1[i] * nums2[i];
        }
        return s;
    }
};
