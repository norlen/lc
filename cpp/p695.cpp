// [695. Max Area of Island](https://leetcode.com/problems/max-area-of-island/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        const pair<int,int> offsets[4] = {{1,0}, {-1,0}, {0,1}, {0,-1}};
        
        if (grid.size() == 0) return 0;
        int n = grid.size();
        int m = grid[0].size();
        
        int max_size = 0;
        stack<pair<int,int>> s;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] == 1) {
                    int size = 0;
                    grid[i][j] = 0;
                    s.push({i,j});
                    while (!s.empty()) {
                        pair<int,int> u = s.top();
                        s.pop();
                        size++;
                        
                        for (pair<int,int> o : offsets) {
                            int vr = u.first + o.first;
                            int vc = u.second + o.second;
                            if (vr < 0 || vr >= n || vc < 0 || vc >= m || grid[vr][vc] != 1) continue;
                            grid[vr][vc] = 0;
                            s.push({vr,vc});
                        }
                    }
                    max_size = max(max_size, size);
                }
            }
        } 
        return max_size;
    }
};
