// [980. Unique Paths III](https://leetcode.com/problems/unique-paths-iii/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    static constexpr int EMPTY = 0;
    static constexpr int START = 1;
    static constexpr int END = 2;
    static constexpr int OBSTACLE = 3;
    static constexpr pair<int,int> offsets[4] = {{1,0}, {-1,0}, {0,1}, {0,-1}};
    
    int uniquePathsIII(vector<vector<int>>& grid) {
        int n = grid.size();
        int m = grid[0].size();
        
        int num_empty = 0;
        pair<int,int> start = {-1, -1};
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] == EMPTY) {
                    num_empty++;
                }
                if (grid[i][j] == START) {
                    start = {i, j};
                }
            }
        }
        
        vector<vector<bool>> vis(n, vector<bool>(m, false));
        
        return dfs(grid, vis, start, 0, num_empty);
    }
    
    int dfs(vector<vector<int>>& grid, vector<vector<bool>>& vis, pair<int,int> u, int depth, int tdepth) {
        int n = grid.size();
        int m = grid[0].size();
        
        if (grid[u.first][u.second] == END) {
            if (depth == tdepth+1) {
                return 1;
            }
            return 0;
        }
        
        if (vis[u.first][u.second]) return 0;
        
        int ways = 0;
        vis[u.first][u.second] = true;
        for (pair<int,int> o : offsets) {
            int vr = u.first + o.first;
            int vc = u.second + o.second;
            if (vr < 0 || vr >= n || vc < 0 || vc >= m) continue;

            if (grid[vr][vc] != EMPTY && grid[vr][vc] != END) continue; // not walkable.
            ways += dfs(grid, vis, {vr, vc}, depth+1, tdepth);
        }
        vis[u.first][u.second] = false;
        return ways;
    }
};
