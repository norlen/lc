// [404. Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves/)
#include "def.h"

class Solution {
public:
    int sumOfLeftLeaves(TreeNode* root) {
        return sum(root, true);
    }
    
    int sum(TreeNode* root, bool r) {
        if (root == nullptr) return 0;
        
        if (root->left == nullptr && root->right == nullptr) {
            return r ? 0 : root->val;
        }
        return sum(root->left, false) + sum(root->right, true);
    }
};
