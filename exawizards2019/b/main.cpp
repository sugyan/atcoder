#include <iostream>

using namespace std;

int main() {
    int n, r = 0, b = 0;
    string s;
    cin >> n >> s;
    for (int i = 0; i < n; ++i) {
        if (s[i] == 'R') ++r;
        if (s[i] == 'B') ++b;
    }
    if (r > b) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}
