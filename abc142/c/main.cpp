#include <iostream>
#include <unordered_map>

using namespace std;

int main() {
    int n;
    cin >> n;
    unordered_map<int, int> um(n);
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        um[a - 1] = i + 1;
    }
    for (int i = 0; i < n; ++i) {
        cout << um[i];
        if (i < n - 1) cout << " ";
    }
    cout << endl;
}
