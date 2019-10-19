#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (int i = 0; i < n; ++i) {
        cin >> v[i];
    }
    sort(v.begin(), v.end());
    long long answer = 0;
    for (int i = 0; i < v.size() - 2; ++i) {
        for (int j = i + 1; j < v.size() - 1; ++j) {
            if (v[i] + v[j] > v.back()) {
                answer += v.size() - j - 1;
            } else {
                for (int k = j + 1; k < v.size(); ++k) {
                    if (v[k] >= v[i] + v[j]) break;
                    ++answer;
                }
            }
        }
    }
    cout << answer << endl;
}
