#include <iostream>
#include <unordered_set>

using namespace std;

int main() {
    int x, n;
    cin >> x >> n;
    unordered_set<int> us;
    for (int i = 0; i < n; ++i) {
        int p;
        cin >> p;
        us.insert(p);
    }
    for (int i = 0; i < 100; ++i) {
        if (us.find(x - i) == us.end()) {
            cout << x - i << endl;
            return 0;
        }
        if (us.find(x + i) == us.end()) {
            cout << x + i << endl;
            return 0;
        }
    }
}
