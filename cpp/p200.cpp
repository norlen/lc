// [200. Number of Islands](https://leetcode.com/problems/number-of-islands/)
#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    void dfs(vector<vector<char>>& grid, int r, int c) {
        stack<pair<int,int>> s;
        s.push({r, c});

        pair<int,int> offsets[4] = {{1,0}, {-1,0}, {0,1}, {0,-1}};
        while (!s.empty()) {
            auto c = s.top();
            s.pop();

            for (int i = 0; i < 4; ++i) {
                int nr = c.first + offsets[i].first;
                int nc = c.second + offsets[i].second;
                if (nr < 0 || nr >= grid.size() || nc < 0 || nc >= grid[0].size()) continue;
                if (grid[nr][nc] == '1') {
                    grid[nr][nc] = '0';
                    s.push({nr, nc});
                }
            }
        }
    }

    int numIslands(vector<vector<char>>& grid) {
        int components = 0;
        for (int i = 0; i < grid.size(); ++i) {
            for (int j = 0; j < grid[0].size(); ++j) {
                if (grid[i][j] == '1') {
                    grid[i][j] = '0';
                    components++;
                    dfs(grid, i, j);
                }
            }
        }
        return components;
    }
};