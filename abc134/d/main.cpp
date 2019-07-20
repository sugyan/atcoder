#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    vector<int> answer;
    vector<int> v(n, 0);
    for (int i = n; i > 1; --i) {
        if (a[i - 1] != v[i - 1]) {
            answer.push_back(i);
            v[i - 1] = 1 - v[i - 1];
            for (int j = 1; j * j <= i; ++j) {
                if (i % j == 0) {
                    v[j - 1] = 1 - v[j - 1];
                    if (j * j != i) {
                        v[i / j - 1] = 1 - v[i / j - 1];
                    }
                }
            }
        }
    }
    if (a[0] != v[0]) answer.push_back(1);

    cout << answer.size() << endl;
    if (!answer.empty()) {
        for (int i = 0, size = answer.size(); i < size; ++i) {
            cout << answer[i];
            if (i < size - 1) cout << " ";
        }
        cout << endl;
    }
}
