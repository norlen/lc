// [357. Count Numbers with Unique Digits](https://leetcode.com/problems/count-numbers-with-unique-digits/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int countNumbersWithUniqueDigits(int n) {
        // 0: 1
        // 1: 9. + prev
        // 2: 9 * 9 + prev
        // 3: 9 * 9 * 8 + prev
        if (n == 0) return 1;
        int curr = 10;
        int mul = 9;
        for (int i = 2; i <= n; ++i) {
            mul *= (9 + 2 - i);
            curr += mul;
        }
        return curr;
    }
};
