// [1231. Divide Chocolate](https://leetcode.com/problems/divide-chocolate/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int maximizeSweetness(vector<int>& sweetness, int k) {
        // Binary search for the answer, over the range [min(sweetness), sum(sweetness)/k+1].
        int l = sweetness[0];
        int r = 0;
        for (auto s : sweetness) {
            l = min(l, s);
            r += s;
        }
        r /= k+1;
        
        while (l < r) {
            int mid = l + (r - l + 1) / 2;
            
            if (cut_works(sweetness, k, mid)) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        return l;
    }
    
    bool cut_works(vector<int>& sweetness, int k, int v) {
        // Try to make cut work with sweetness v.
        int pieces = 0;
        int c = 0;
        for (auto s : sweetness) {
            c += s;
            if (c >= v) {
                pieces++;
                c = 0;
            }
        }
        return pieces >= (k+1);
    }
};
