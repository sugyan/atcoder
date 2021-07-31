#include <iostream>
#include <vector>
#include <algorithm>
#include <iomanip>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (int i = 0; i < n; ++i) {
        cin >> v[i];
    }
    sort(v.begin(), v.end());
    double answer = v[0];
    for (int i = 1;i < n; ++i) {
        answer = (answer + v[i]) / 2.0;
    }
    cout << setprecision(9) << answer << endl;
}
