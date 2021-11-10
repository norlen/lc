// [733. Flood Fill](https://leetcode.com/problems/flood-fill/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int newColor) {
        const pair<int,int> offsets[4] = {{1,0}, {-1,0}, {0,1}, {0,-1}};
        
        int base_color = image[sr][sc];
        if (base_color == newColor) {
            return image;
        }
        
        deque<pair<int,int>> q;
        q.push_back({sr,sc});
        image[sr][sc] = newColor;
        
        while (!q.empty()) {
            pair<int,int> u = q.front();
            q.pop_front();
            
            for (pair<int,int> o : offsets) {
                int vr = u.first + o.first;
                int vc = u.second + o.second;
                if (vr < 0 || vr >= image.size() || vc < 0 || vc >= image[0].size()) {
                    continue;
                }
                if (image[vr][vc] == base_color) {
                    image[vr][vc] = newColor;
                    q.push_back({vr, vc});
                }
            }
        }
        return image;
    }
};
