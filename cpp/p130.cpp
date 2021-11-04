#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    void solve(vector<vector<char>>& board) {
        int n = board.size();
        int m = board[0].size();
        
        // Check borders.
        for (int i = 0; i < n; ++i) {
            if (board[i][0] == 'O') dfs(board, i, 0);
            if (board[i][m-1] == 'O') dfs(board, i, m-1);
        }
        for (int i = 0; i < m; ++i) {
            if (board[0][i] == 'O') dfs(board, 0, i);
            if (board[n-1][i] == 'O') dfs(board, n-1, i);
        }
        
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (board[i][j] == 'O') board[i][j] = 'X';
                else if (board[i][j] == 'Q') board[i][j] = 'O';
            }
        }
    }
    
    void dfs(vector<vector<char>>& board, int i, int j) {
        stack<pair<int,int>> s;
        s.push({i, j});
        const pair<int,int> offsets[4] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
        while (!s.empty()) {
            auto p = s.top();
            s.pop();
            board[p.first][p.second] = 'Q';
            for (auto o : offsets) {
                int x = o.first + p.first;
                int y = o.second + p.second;
                if (x > 0 && x < board.size() &&
                    y > 0 && y < board[0].size() &&
                    board[x][y] == 'O') {
                    s.push({x, y});
                }
            }
        }
    }
};