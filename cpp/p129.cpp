/**
 * Definition for a binary tree node.
 */
 struct TreeNode {
     int val;
     TreeNode *left;
     TreeNode *right;
     TreeNode() : val(0), left(nullptr), right(nullptr) {}
     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 };

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