#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    int answer = 0;
    for (int i = 0; i < s.length() / 2; i++) {
        if (s[i] != s[s.length() - 1 - i]) ++answer;
    }
    cout << answer << endl;
}
