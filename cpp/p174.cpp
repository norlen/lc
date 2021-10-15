// [174. Dungeon Game](https://leetcode.com/problems/dungeon-game/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int calculateMinimumHP(vector<vector<int>>& dungeon) {
        // return rec_with_memo(dungeon);
        // return dp_iter(dungeon);
        return dp_iter2(dungeon);
    }
    
    int rec_with_memo(vector<vector<int>>& dungeon) {
        int w = dungeon[0].size();
        int h = dungeon.size();
        unordered_map<int, int> memo;
        return max(1 - rec(dungeon, 0, 0, w, h, memo), 1);
    }
    
    int rec(vector<vector<int>>& dungeon, int x, int y, int w, int h, unordered_map<int, int>& memo) {
        int idx = y * w + x;
        if (memo.count(idx) > 0) {
            return memo[idx];
        }
        if (x == w-1 && y == h-1) {
            return dungeon[y][x];
        }
        if (x >= w || y >= h) {
            return INT_MIN;
        }
        
        int m = min(rec(dungeon, x+1, y, w, h, memo), 0);
        int n = min(rec(dungeon, x, y+1, w, h, memo), 0);
        int a = max(m, n) + dungeon[y][x];
        memo[idx] = a;
        return memo[idx];
    }
    
    int dp_iter2(vector<vector<int>>& dungeon) {
        int w = dungeon[0].size();
        int h = dungeon.size();
        
        vector<vector<int>> dp(2, vector<int>(w+1, INT_MIN));
        dp[1][w-1] = 0;
        
        int a = 0;
        int b = 1;
        for (int i = h-1; i >= 0; --i) {
            for (int j = w-1; j >= 0; --j) {
                int x = min(dp[a][j+1], 0);
                int y = min(dp[b][j], 0);
                dp[a][j] = dungeon[i][j] + max(x, y);
            }
            swap(a, b);
        }
        
        return max(1, 1 - dp[b][0]);
    }
    
    int dp_iter(vector<vector<int>>& dungeon) {
        int w = dungeon[0].size();
        int h = dungeon.size();
        
        vector<vector<int>> dp(h+1, vector<int>(w+1, INT_MIN));
        dp[h][w-1] = 0;
        
        for (int i = h-1; i >= 0; --i) {
            for (int j = w-1; j >= 0; --j) {
                int a = min(dp[i+1][j], 0);
                int b = min(dp[i][j+1], 0);
                dp[i][j] = dungeon[i][j] + max(a, b);
            }
        }
        
        return max(1, 1 - dp[0][0]);
    }
};