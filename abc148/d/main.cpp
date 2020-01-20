#include <iostream>

using namespace std;

int main() {
    int n, k = 1, answer = 0;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        int a = 0;
        cin >> a;
        if (a == k) {
            ++k;
        } else {
            ++answer;
        }
    }
    cout << (answer == n ? -1 : answer) << endl;
}
