#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    int answer = 0;
    for (int i = 0; i < 3; ++i) {
        if (s[i] == 'R') {
            ++answer;
            for (int j = i + 1; j < 3; ++j) {
                if (s[j] == 'R') {
                    ++answer;
                } else {
                    break;
                }
            }
            break;
        }
    }
    cout << answer << endl;
}
