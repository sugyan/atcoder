#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    bool yes = true;
    for (int i = 0, len = s.length(); i < len; ++i) {
        if ((i % 2 == 0 && s[i] == 'L') || (i % 2 == 1 && s[i] == 'R')) {
            yes = false;
            break;
        }
    }
    cout << (yes ? "Yes" : "No") << endl;
}
