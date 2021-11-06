// [91. Decode Ways](https://leetcode.com/problems/decode-ways/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    unordered_map<int,int> m;
    
    int numDecodings(string s) {
        if (s[0] == '0') return 0;
        //return ways(s, 0);
        // return iter(s);
        return iter_constant_space(s);
    }
    
    int iter_constant_space(string& s) {
        int a = 1;
        int b = 1;
        
        for (int i = 1; i < s.size(); ++i) {
            int next = 0;
            if (s[i] != '0') {
                next += b;
            }
            if (s[i-1] == '1' || s[i-1] == '2' && s[i] <= '6') {
                next += a;
            }
            a = b;
            b = next;
        }
        return b;
    }
    
    int iter(string& s) {
        vector<int> dp(s.size() + 1);
        dp[0] = 1;
        dp[1] = 1;
        
        for (int i = 2; i < dp.size(); ++i) {
            if (s[i-1] != '0') {
                dp[i] = dp[i - 1];
            }
            if (s[i-2] == '1' || s[i-2] == '2' && s[i-1] <= '6') {
                dp[i] += dp[i - 2];
            }
        }
        return dp[dp.size()-1];
    }

    int ways(string& s, int i) {
        if (m.count(i) > 0) {
            return m[i];
        }
        
        if (i == s.size()) {
            return 1;
        }
        if (s[i] == '0') {
            return 0;
        }
        
        int w = ways(s, i+1);
        if ((s.size() - i) >= 2 && (s[i] == '1' || s[i] == '2' && s[i+1] <= '6')) {
            w += ways(s, i+2);
        }
        m.insert({i, w});
        return w;
    }
};