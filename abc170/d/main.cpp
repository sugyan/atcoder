#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    sort(a.begin(), a.end());
    vector<bool> v(a.back(), true);
    for (int e : a) {
        for (size_t i = 2; e * i - 1 < v.size(); ++i) {
            v[e * i - 1] = false;
        }
    }
    int answer = 0;
    for (size_t i = 0; i < a.size(); ++i) {
        if (i + 1 < a.size() && a[i + 1] == a[i]) {
            int m = a[i];
            while (i < a.size() && a[i + 1] == m) ++i;
        } else if (v[a[i] - 1]) {
            ++answer;
        }
    }
    cout << answer << endl;
}
