#include <deque>
#include "def.h"

using namespace std;

class Solution {
public:
    Node* connect(Node* root) {
        // Not using constant space works exactly the same as p116. So let's only do O(1) space.
        if (root == nullptr) return root;

        auto level = root;
        while (level != nullptr) {
            auto head = level;

            level = nullptr;
            Node* last = nullptr;
            while (head != nullptr) {
                // Find next level.
                if (level == nullptr) {
                    level = head->left != nullptr ? head->left : head->right;
                }

                // Fix link to previous subtree to this.
                if (last != nullptr && head->left != nullptr) {
                    last->next = head->left;
                } else if (last != nullptr && head->right != nullptr) {
                    last->next = head->right;
                }

                if (head->left != nullptr && head->right != nullptr) {
                    head->left->next = head->right;
                    last = head->right;
                } else if (head->left != nullptr) {
                    last = head->left;
                } else if (head->right != nullptr) {
                    last = head->right;
                }
                head = head->next;
            }
        }

        return root;
    }
};
