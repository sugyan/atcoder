#include <iostream>

using namespace std;

int main() {
    string s, t;
    cin >> s >> t;
    int answer = 0;
    for (int i = 0; i < 3; ++i) {
        if (s[i] == t[i]) ++answer;
    }
    cout << answer << endl;
}
