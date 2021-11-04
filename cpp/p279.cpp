// [279. Perfect Squares](https://leetcode.com/problems/perfect-squares/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numSquares(int n) {
        //return recursive(n);
        //return iter(n);
        // return greedy_enumeration(n);
        return greedy_enumeration_bfs(n);
    }
    
    int iter(int n) {
        vector<int> dp(n+1);
        for (int i = 0; i < dp.size(); ++i) {
            dp[i] = INT_MAX;
        }
        dp[0] = 0;
        
        // All perfect squares <= n.
        vector<int> sq;
        int i = 1;
        while (i*i <= n) {
            sq.push_back(i*i);
            i += 1;
        }
        
        for (int i = 1; i < dp.size(); ++i) {
            for (int j = 0; j < sq.size(); ++j) {
                int s = sq[j];
                if (s > i) break;
                
                if (dp[i-s] != INT_MAX) {
                    dp[i] = min(dp[i], dp[i-s] + 1);
                }
            }
        }
        return dp[n];
    }
    
    int recursive(int n) {
        // All perfect square <= n.
        vector<int> sq;
        int i = 1;
        while (i*i <= n) {
            sq.push_back(i*i);
            i += 1;
        }
        vector<int> memo(n+1);
        for (int i = 0; i < n+1; ++i) {
            memo[i] = -1;
        }
        return rec(n, sq, memo);
    }
    
    int rec(int n, vector<int>& sq, vector<int>& memo) {
        if (n < 0) {
            return -1;
        }
        if (memo[n] != -1) {
            return memo[n];
        }
        if (n == 0) {
            return 0;
        }
        
        int m = INT_MAX;
        for (int i = sq.size()-1; i >= 0; --i) {
            int k = rec(n - sq[i], sq, memo);
            if (k != -1) {
                m = min(m, k + 1);
            }
        }
        memo[n] = m;
        return m;
    }
    
    // From solution: approach 3.
    int greedy_enumeration(int n) {
        unordered_set<int> s;
        for (int i = 0; i < sqrt(n)+1; ++i) {
            s.insert(i*i);
        }
        
        for (int i = 1; i <= n; ++i) {
            if (is_divided_by(n, i, s)) {
                return i;
            }
        }
        return 0;
    }
    
    bool is_divided_by(int n, int count, unordered_set<int>& s) {
        if (count == 1) {
            return s.count(n) > 0;
        }
        
        for (int sq : s) {
            if (is_divided_by(n - sq, count - 1, s)) {
                return true;
            }
        }
        return false;
    }
    
    // From solution: approach 4.
    int greedy_enumeration_bfs(int n) {
        vector<int> sq;
        for (int i = 0; i < sqrt(n)+1; ++i) {
            sq.push_back(i*i);
        }
        
        unordered_set<int> q;
        q.insert(n);
        
        int c = 0;
        while (!q.empty()) {
            c += 1;
            unordered_set<int> next_q;
            
            for (int u : q) {
                for (int s : sq) {
                    if (s > u) break;
                    if (u == s) {
                        return c;
                    }
                    next_q.insert(u - s);
                }
            }
            
            q = next_q;
        }
        return 0;
    }
};
