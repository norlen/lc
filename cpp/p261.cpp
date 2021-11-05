// [261. Graph Valid Tree](https://leetcode.com/problems/graph-valid-tree/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    bool validTree(int n, vector<vector<int>>& edges) {
        // A graph is a tree if it is connected and contains no cycles.
        // One property for a tree is that V = E + 1.
        if (n != edges.size() + 1) {
            return false;
        }
        
        vector<vector<int>> e(n);
        for (int i = 0; i < edges.size(); ++i) {
            int u = edges[i][0];
            int v = edges[i][1];
            e[u].push_back(v);
            e[v].push_back(u);
        }
        
        // Check if it's connected.
        int num_visits = 0;
        vector<bool> vis(n, false);
        stack<int> s;
        
        s.push(0); // Start anywhere.
        vis[0] = true;
        while (!s.empty()) {
            int u = s.top();
            s.pop();
            num_visits++;
            
            for (int v : e[u]) {
                if (!vis[v]) {
                    vis[v] = true;
                    s.push(v);
                }
            }
        }
        
        return num_visits == n;
    }
};
