// [338. Counting Bits](https://leetcode.com/problems/counting-bits/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> countBits(int n) {
        // For number n it has the same bit pattern as n >> 1, with the exception of the
        // least significant bit, which is 1 if odd, otherwise 0.
        vector<int> bits(n+1);
        bits[0] = 0;
        for (int i = 1; i < bits.size(); ++i) {
            bits[i] = bits[i >> 1] + i % 2;
        }
        return bits;
    }
};
