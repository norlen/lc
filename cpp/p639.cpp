// [639. Decode Ways II](https://leetcode.com/problems/decode-ways-ii/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numDecodings(string s) {
        if (s.size() == 0 || s[0] == '0') return 0;
        long long a = 1;
        long long b = s[0] == '*' ? 9 : 1;
        
        for (int i = 1; i < s.size(); ++i) {
            char c = s[i];
            char p = s[i-1];
            bool w = c == '*';
            long long next = 0;
            if (c != '0') {
                next = w ? b*9 : b;
            }
            if (p == '1' || p == '*') {
                next += w ? a*9 : a;
            }
            if ((p == '2' || p == '*') && (c <= '6' || c == '*')) {
                next += w ? a*6 : a;
            }
            next %= 1000000007;
            swap(a, b);
            b = next;
        }
        return b;
    }
};