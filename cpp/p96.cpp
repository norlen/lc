// [96. Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees/)
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    unordered_map<int,int> m;
    
    int numTrees(int n) {
        //return rec(n);
        return iter(n);
    }
    
    int iter(int n) {
        vector<int> dp(n+1);
        dp[0] = 1;
        dp[1] = 1;
        
        for (int i = 2; i <= n; ++i) {
            for (int j = 1; j <= i; ++j) {
                dp[i] += dp[j-1] * dp[i-j];
            }
        }
        return dp[n];
    }
    
    int rec(int n) {
        if (m.count(n) > 0) {
            return m[n];
        }
        if (n <= 1) {
            return 1;
        }
        int c = 0;
        for (int i = 1; i <= n; ++i) {
            c += rec(i-1) * rec(n-i);
        }
        m.insert({n, c});
        return c;
    }
};