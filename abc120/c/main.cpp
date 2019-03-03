#include <iostream>
#include <stack>

using namespace std;

int main() {
    string s;
    stack<char> t;
    cin >> s;
    for (string::iterator it = s.begin(); it != s.end(); ++it) {
        if (!t.empty() && t.top() != *it) {
            t.pop();
        } else {
            t.push(*it);
        }
    }
    cout << s.size() - t.size() << endl;
}