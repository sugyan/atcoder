#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    int answer = 0;
    if (n >= 1) answer += min(9, n);
    if (n >= 100) answer += min(900, n - 100 + 1);
    if (n >= 10000) answer += min(90000, n - 10000 + 1);
    cout << answer << endl;
}
