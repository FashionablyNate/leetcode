#include <iostream>

class Solution {
public:
    std::string convert(std::string s, int numRows) {

        if (numRows == 1) return s;

        std::string buffer;
        int n = s.size();
        int stride = 2 * numRows - 2;

        for (int row = 0; row < numRows; row++) {
            for (int j = 0; j + row < n; j += stride) {
                buffer += s[j + row];
                if (row != 0 && row != numRows - 1 && j + stride - row < n)
                    buffer += s[j + stride - row];
            }
        }
        return buffer;
    }
};

int main() {
    Solution solution;
    std::cout << solution.convert("PAYPALISHIRING", 3) << std::endl;
    std::cout << solution.convert("PAYPALISHIRING", 4) << std::endl;
}