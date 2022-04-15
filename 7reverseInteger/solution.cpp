#include <cmath>
#include <iostream>

class Solution {
public:
    int reverse(int x) {
        bool negative = false; long y = x;
        if (y < 0) { y *= -1; negative = true; }
        if ((y > pow(2, 31) - 1) || (y == 0)) return 0;
        long result = 0; int max = log10(y);
        for (long i = 0, j = 1, k = pow(10, max); i < max + 1; i++, j *= 10, k /= 10) {
            result += y / j % 10 * k;
        }
        
        if (result > pow(2, 31) - 1) return 0;
        return ((negative) ? result * -1 : result);
    }
};

int main() {
    Solution solution;
    std::cout << solution.reverse(123) << std::endl;
    std::cout << solution.reverse(-123) << std::endl;
    std::cout << solution.reverse(120) << std::endl;
}