// [309. Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        // return recursion_with_memo(prices);
        // return iterative_dp(prices);
        return state_machine_dp(prices);
    }
    
    int iterative_dp(vector<int>& prices) {
        vector<int> dp(prices.size() + 2);
        dp[dp.size()-1] = 0;
        dp[dp.size()-2] = 0;
        
        int e = prices.size() + 2;
        for (int i = prices.size()-1; i >= 0; --i) {
            int max_profit = 0;
            for (int j = i+1; j < prices.size(); ++j) {
                int profit = prices[j] - prices[i] + dp[j+2];
                max_profit = max(max_profit, profit);
            }
            dp[i] = max(dp[i], max(dp[i+1], max_profit));
        }
        return dp[0];
    }
    
    int recursion_with_memo(vector<int>& prices) {
        vector<int> memo(prices.size());
        for (int i = 0; i < memo.size(); i++) memo[i] = -1;
        
        return r(prices, 0, memo);
    }
    
    int r(vector<int>& prices, int i, vector<int>& memo) {
        if (i > prices.size()-1) {
            return 0;
        }
        if (memo[i] != -1) {
            return memo[i];
        }
        
        int p = prices[i];
        int max_profit = 0;
        for (int j = i+1; j < prices.size(); ++j) {
            int profit = prices[j] - p + r(prices, j + 2, memo);
            max_profit = max(max_profit, profit);
        }
        int nobuy = r(prices, i+1, memo);
        memo[i] = max(max_profit, nobuy);
        return memo[i];
    }
    
    // From solution
    int state_machine_dp(vector<int>& prices) {
        int sold = INT_MIN;
        int held = INT_MIN;
        int reset = 0;
        
        for (auto price : prices) {
            int new_sold = held + price;
            held = max(held, reset - price);
            reset = max(reset, sold);
            sold = new_sold;
        }
        
        return max(sold, reset);
    }
};