// [207. Course Schedule](https://leetcode.com/problems/course-schedule/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> e(numCourses);
        vector<int> indegree(numCourses, 0);
        
        for (int i = 0; i < prerequisites.size(); ++i) {
            int u = prerequisites[i][0];
            int v = prerequisites[i][1];
            e[u].push_back(v);
            indegree[v]++;
        }
        
        stack<int> s;
        for (int i = 0; i < indegree.size(); ++i) {
            if (indegree[i] == 0) {
                s.push(i);
            }
        }
        
        int num_visits = 0;
        while (!s.empty()) {
            num_visits++;
            int u = s.top();
            s.pop();
            
            for (int v : e[u]) {
                indegree[v]--;
                if (indegree[v] == 0) {
                    s.push(v);
                }
            }
        }
        
        return num_visits == numCourses;
    }
};
