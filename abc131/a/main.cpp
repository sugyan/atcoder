#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    bool good = true;
    for (int i = 0; i < 3; ++i) {
        if (s[i] == s[i + 1]) good = false;
    }
    cout << (good ? "Good" : "Bad") << endl;
}
