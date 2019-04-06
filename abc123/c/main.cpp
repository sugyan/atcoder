#include <bits/stdc++.h>

using namespace std;

int main() {
    unsigned long long n, answer;
    vector<unsigned long long> t(5, 0);
    cin >> n;
    for (int i = 0; i < 5; ++i) {
        cin >> t[i];
    }
    sort(t.begin(), t.end());
    answer = n / t.front() + 4;
    if (n % t.front() != 0) ++answer;
    cout << answer << endl;
}
