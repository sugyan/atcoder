#include <bits/stdc++.h>

using namespace std;

int main() {
    string s;
    cin >> s;
    int n1 = 0, n2 = 0;
    for (int i = 0, l = s.length(); i < l; ++i) {
        if (i % 2 == 0) {
            if (s[i] == '0') ++n1;
            if (s[i] == '1') ++n2;
        } else {
            if (s[i] == '1') ++n1;
            if (s[i] == '0') ++n2;
        }
    }
    cout << min(n1, n2) << endl;
}
