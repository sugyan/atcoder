#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    int count = 0;
    for (int i = 0; i < n; ++i) {
        int p;
        cin >> p;
        if (p != i + 1) ++count;
    }
    cout << (count <= 2 ? "YES" : "NO") << endl;
}
