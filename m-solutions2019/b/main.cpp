#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    int lose = 0;
    for (char c : s) {
        if (c == 'x') {
            if (++lose > 7) {
                cout << "NO" << endl;
                return 0;
            };
        }
    }
    cout << "YES" << endl;
}
