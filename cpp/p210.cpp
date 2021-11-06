// [210. Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> e(numCourses);
        vector<int> indegree(numCourses, 0);
        
        for (int i = 0; i < prerequisites.size(); ++i) {
            int u = prerequisites[i][1];
            int v = prerequisites[i][0];
            e[u].push_back(v);
            indegree[v]++;
        }
        
        stack<int> s;
        for (int i = 0; i < indegree.size(); ++i) {
            if (indegree[i] == 0) {
                s.push(i);
            }
        }
        
        vector<int> ts;
        while (!s.empty()) {
            int u = s.top();
            s.pop();
            ts.push_back(u);
            
            for (auto v : e[u]) {
                --indegree[v];
                if (indegree[v] == 0) {
                    s.push(v);
                }
            }
        }
        
        if (ts.size() != numCourses) {
            ts.clear();
        }
        return ts;
    }
};
