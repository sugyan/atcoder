#include <iostream>

using namespace std;

int main() {
    int n;
    string s, t;
    cin >> n >> s >> t;
    for (int i; i < n; ++i) {
        cout << s[i] << t[i];
    }
    cout << endl;
}
