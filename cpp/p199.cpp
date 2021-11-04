#include <bits/stdc++.h>
#include "def.h"

using namespace std;

class Solution {
public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> sol;
        solve(root, 0, sol);
        return sol;
    }
    
    void solve(TreeNode* node, int depth, vector<int>& sol) {
        if (node == nullptr) return;
        if (depth >= sol.size()) {
            sol.push_back(node->val);
        } else {
            sol[depth] = node->val;
        }
        solve(node->left, depth + 1, sol);
        solve(node->right, depth + 1, sol);
    }
};