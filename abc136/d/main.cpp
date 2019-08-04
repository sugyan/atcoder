#include <iostream>
#include <vector>

using namespace std;

int main() {
    string s;
    cin >> s;
    vector<int> answer(s.length(), 0);
    int r = 0, l = 0;
    for (int i = 0, length = s.length(); i < length; ++i) {
        if (s[i] == 'R') ++r;
        if (i > 0 && s[i - 1] == 'R' && s[i] == 'L') {
            while (i + l < length && s[i + l] == 'L') ++l;
            answer[i - 1] = r / 2 + l / 2 + (r % 2);
            answer[i] = r / 2 + l / 2 + (l % 2);
            i += l - 1;
            r = 0, l = 0;
        }
    }
    for (int i = 0, size = answer.size(); i < size; ++i) {
        cout << answer[i];
        if (i < size - 1) cout << " ";
    }
    cout << endl;
}
