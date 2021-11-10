// [329. Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int n = matrix.size();
        int m = matrix[0].size();
        vector<vector<int>> p(n, vector<int>(m, -1));
    
        int ans = 0;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                ans = max(ans, dfs({i,j}, matrix, p));
            }
        }
        return ans;
  }
    
    int dfs(pair<int,int> s, vector<vector<int>>& g, vector<vector<int>>& p) {
        int a = s.first;
        int b = s.second;
        if (p[a][b] != -1) {
            return p[a][b];
        }
            
        int n = g.size();
        int m = g[0].size();
        int v = g[a][b];
        int lp = 1;
        pair<int,int> off[4] = {{1,0}, {-1,0}, {0,1}, {0,-1}};
        for (pair<int,int> o : off) {
            int r = a + o.first;
            int c = b + o.second;
            if (r < 0 || r >= n || c < 0 || c >= m || v >= g[r][c]) continue;
            lp = max(lp, dfs({r,c}, g, p) + 1);
        }
            
        p[a][b] = lp;
        return lp;
    }
};
