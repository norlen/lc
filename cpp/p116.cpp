#include <bits/stdc++.h>
#include "def.h"
using namespace std;

class Solution {
public:
    Node* connect(Node* root) {
        deque<Node*> q;
        if (root != nullptr) {
            q.push_back(root);
        }

        while (!q.empty()) {
            Node* prev = nullptr;
            auto nodes_at_level = q.size();
            for (auto i = 0; i < nodes_at_level; ++i) {
                auto n = q.front();
                q.pop_front();

                if (prev != nullptr) {
                    prev->next = n;
                }
                prev = n;
                
                if (n->left != nullptr) q.push_back(n->left);
                if (n->right != nullptr) q.push_back(n->right);
            }
        }

        return root;
    }

    Node* connect_constant_space(Node* root) {
        if (root == nullptr) return root;

        auto level = root;
        while (level->left != nullptr) {
            auto head = level;
            
            while (head != nullptr) {
                head->left->next = head->right;
                if (head->next != nullptr) {
                    head->right->next = head->next->left;
                }
                head = head->next;
            }

            // Start of next level.
            level = level->left;
        }

        return root;
    }
};
