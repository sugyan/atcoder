#include <iostream>

using namespace std;

int main() {
    int n;
    string s;
    cin >> n >> s;
    char prev;
    int answer = 0;
    for (char& c : s) {
        if (c != prev) ++answer;
        prev = c;
    }
    cout << answer << endl;
}
