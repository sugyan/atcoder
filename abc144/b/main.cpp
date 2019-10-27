#include <iostream>
#include <unordered_set>

using namespace std;

int main() {
    unordered_set<int> s;
    for (int i = 0; i < 9; ++i) {
        for (int j = 0; j < 9; ++j) {
            s.insert((i + 1) * (j + 1));
        }
    }
    int n;
    cin >> n;
    cout << (s.find(n) != s.end() ? "Yes" : "No") << endl;
}
