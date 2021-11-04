#include "def.h"

class Solution {
public:
    int sumNumbers(TreeNode* root) {
        return dfs(root, 0);
    }

    int dfs(TreeNode* root, int current) {
        if (root == nullptr) {
            return 0;
        }
        
        current *= 10;
        current += root->val;
        if (root->left == nullptr && root->right == nullptr) {
            return current;
        } else {
            return dfs(root->left, current) + dfs(root->right, current);
        }
    }
};