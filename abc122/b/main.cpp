#include <iostream>
#include <cmath>

using namespace std;

int main() {
    string s;
    cin >> s;
    int result = 0;
    int count = 0;
    for (string::iterator it = s.begin(); it != s.end(); ++it) {
        if (*it == 'A' || *it == 'T' || *it == 'C' || *it == 'G') {
            ++count;
        } else {
            result = max(result, count);
            count = 0;
        }
    }
    result = max(result, count);
    cout << result << endl;
}