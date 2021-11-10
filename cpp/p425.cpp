// [425. Word Squares](https://leetcode.com/problems/word-squares/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<vector<string>> wordSquares(vector<string>& words) {
        map<string, vector<string>> prefixes;
        for (string& word: words) {
            string prefix = "";
            for (int i = 0; i < word.size()-1; ++i) {
                prefix.push_back(word[i]);
                if (prefixes.count(prefix) == 0) {
                    prefixes.insert({prefix, vector<string>()});
                }
                prefixes[prefix].push_back(word);
            }
        }
        
        
        vector<vector<string>> ans;
        vector<string> curr;
        for (string& word : words) {
            curr.push_back(word);
            bt(words, word.size(), 1, curr, ans, prefixes);
            curr.pop_back();
        }
        return ans;
    }
    
    void bt(vector<string>& words, int k, int s, vector<string>& curr, vector<vector<string>>& ans, map<string, vector<string>>& prefixes) {
        if (curr.size() == k) {
            ans.push_back(curr);
            return;
        }
        
        string prefix = "";
        for (int i = 0; i < s; ++i) {
            prefix.push_back(curr[i][s]);
        }
        
        if (prefixes.count(prefix) == 0) return;
        auto& possible_words = prefixes.at(prefix);
        
        for (string& word : possible_words) {
            if (word.size() != k) continue;
            
            curr.push_back(word);
            bt(words, k, s+1, curr, ans, prefixes);
            curr.pop_back();
        }
    }
};
