// [557. Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    string reverseWords(string s) {
        int i = 0;
        while (i < s.size()) {
            if (s[i] == ' ') {
                i++;
                continue;
            }
            
            // i is at a character, find next whitespace.
            int j = i + 1;
            while (j < s.size() && s[j] != ' ') j++;
            int next = j;
            
            // Reverse i to next-1
            j--;
            while (i < j) {
                swap(s[i], s[j]);
                i++;
                j--;
            }
            i = next;
        }
        return s;
    }
};
