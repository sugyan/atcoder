#include <iostream>
#include <unordered_map>

using namespace std;

int main() {
    int n;
    cin >> n;
    unordered_map<int, int> um;
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        ++um[a];
    }
    pair<int, int> begin = *um.begin();
    if (--begin.second == 0) um.erase(begin.first);
    for (pair<int, int> p : um) {
        int i1 = begin.first;
        int i2 = p.first;
        unordered_map<int, int> umm;
        ++umm[p.first];

        bool ok = true;
        for (int j = 0; j < n - 1; ++j) {
            int i = i1 ^ i2;
            i1 = i2;
            i2 = i;
            if (um.find(i) != um.end()) {
                if (++umm[i] > um[i]) {
                    ok = false;
                    break;
                };
            } else {
                if (j == n - 2 && i == begin.first) {
                } else {
                    ok = false;
                }
                break;
            }
        }
        if (ok) {
            cout << "Yes" << endl;
            return 0;
        }
    }
    cout << "No" << endl;
}
