// [343. Integer Break](https://leetcode.com/problems/integer-break/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int integerBreak(int n) {
        // return rr(n);
        return dp_iter(n);
    }
    
    int rr(int n) {
        vector<int> memo(n+1, -1);
        memo[0] = 1;
        memo[1] = 1;
        return r(n, memo);
    }
    
    int r(int n, vector<int>& memo) {
        if (n < 0) {
            return 0;
        } else if (memo[n] != -1) {
            return memo[n];
        }
        
        int max_p = 0;
        for (int i = 1; i < n; ++i) {
            int curr_p = i * max(n - i, r(n - i, memo));
            max_p = max(max_p, curr_p);
        }
        memo[n] = max_p;
        return max_p;
    }
    
    int dp_iter(int n) {
        vector<int> dp(n+1, 1);
        
        for (int i = 2; i <= n; ++i) {
            for (int j = 1; j < i; ++j) {
                int a = max(j, dp[j]);
                int b = max(i-j, dp[i-j]);
                dp[i] = max(dp[i], a*b);
            }
        }
        return dp[n];
    }
};
