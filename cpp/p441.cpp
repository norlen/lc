// [441. Arranging Coins](https://leetcode.com/problems/arranging-coins/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int arrangeCoins(int n) {
        return static_cast<int>(-0.5 + sqrt(0.25 + 2*static_cast<long long>(n)));
    }
};
