// [269. Alien Dictionary](https://leetcode.com/problems/alien-dictionary/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    string alienOrder(vector<string>& words) {
        // Generate graph of the lexicographical order. Mapping a-z to 0-25.
        // Then perform a topological sort on that.
        int n = 26;
        vector<unordered_set<int>> e(n);
        vector<int> indegree(n, 0);
        vector<bool> seen(n, false);
        
        // These are the characters in the alphabet.
        for (auto& w : words) {
            for (auto ch : w) {
                seen[ch - 'a'] = true;
            }
        }
        
        // Create edges.
        for (int i = 1; i < words.size(); ++i) {
            // Find first character that differs and create an edge for that.
            
            if (words[i-1].size() > words[i].size()) {
                bool is_prefix = true;
                for (int j = 0; j < words[i].size(); ++j) {
                    if (words[i-1][j] != words[i][j]) {
                        is_prefix = false;
                        break;
                    }
                }
                if (is_prefix) return "";
            }
            
            int end = min(words[i].size(), words[i-1].size());
            int j = 0;
            while (j < end && words[i][j] == words[i-1][j]) j++;
            
            if (j < end) {
                int u = words[i-1][j] - 'a';
                int v = words[i][j] - 'a';
                if (e[u].count(v) == 0) {
                    indegree[v]++;
                }
                e[u].insert(v);
            }
        }
        
        // Topological sort.
        
        stack<int> s;
        for (int i = 0; i < indegree.size(); ++i) {
            if (seen[i] && indegree[i] == 0) {
                s.push(i);
            }
        }
        
        string top_sort;
        while (!s.empty()) {
            int u = s.top();
            s.pop();
            top_sort.push_back(u + 'a');
            
            for (auto v : e[u]) {
                indegree[v]--;
                if (indegree[v] == 0) {
                    s.push(v);
                }
            }
        }
        
        int num_seen = 0;
        for (auto s : seen) {
            if (s) num_seen++;
        }
        
        if (top_sort.size() != num_seen) {
            return "";
        } else {
            return top_sort;
        }
    }
};
