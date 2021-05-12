#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    int a[26] { 0 };
    for (char c : s) {
        ++a[c - 'A'];
    }
    bool answer = true;
    for (int i = 0; i < 26; ++i) {
        if (a[i] != 0 && a[i] != 2) {
            answer = false;
            break;
        }
    }
    cout << (answer ? "Yes" : "No") << endl;
}
