// [133. Clone Graph](https://leetcode.com/problems/clone-graph/)
#include <bits/stdc++.h>

using namespace std;

class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (node == nullptr) return nullptr;

        unordered_map<int, Node*> new_nodes;
        deque<Node*> q;
        q.push_back(node);
        new_nodes.insert({node->val, new Node(node->val)});

        while (q.size() > 0) {
            auto u = q.front();
            q.pop_front();
            auto nu = new_nodes[u->val];

            for (auto& v : u->neighbors) {
                if (new_nodes.count(v->val) == 0) {
                    new_nodes.insert({v->val, new Node(v->val)});
                    q.push_back(v);
                }
                nu->neighbors.push_back(new_nodes[v->val]);
            }
        }
        return new_nodes[node->val];
    }
};