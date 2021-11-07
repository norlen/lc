// [43. Multiply Strings](https://leetcode.com/problems/multiply-strings/)
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    string multiply(string num1, string num2) {
        if (num1 == "0" || num2 == "0") return "0";
        
        vector<vector<int>> int_res;
        
        reverse(num1.begin(), num1.end());
        reverse(num2.begin(), num2.end());
        
        for (int i = 0; i < num2.size(); ++i) {
            int a = num2[i] - '0';
            
            vector<int> r(i, 0);
            int carry = 0;
            for (int j = 0; j < num1.size(); ++j) {
                int b = num1[j] - '0';
                int res = a * b + carry;
                carry = res / 10;
                r.push_back(res % 10);
            }
            if (carry > 0) {
                r.push_back(carry);
            }
            int_res.push_back(r);
        }
        
        vector<int> res;
        for (vector<int> r : int_res) {
            int carry = 0, i;
            for (i = 0; i < r.size(); ++i) {
                if (res.size() <= i) {
                    res.push_back(0);
                }
                int n = res[i] + r[i] + carry;
                carry = n / 10;
                res[i] = n % 10;
            }
            if (carry > 0) {
                if (res.size() <= i) {
                    res.push_back(carry);
                } else {
                    res[i] = carry;
                }
            }
        }
        
        string ans;
        for (int i = res.size()-1; i >= 0; --i) {
            ans.push_back(res[i] + '0');
        }
        return ans;
    }
};
