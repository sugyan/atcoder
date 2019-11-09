#include <iostream>
#include <map>
#include <vector>

#define DIV 998244353

using namespace std;

int pow(int a, int b) {
    long long ret = a;
    for (int i = 0; i < b - 1; ++i) {
        ret = (ret * a) % DIV;
    }
    return ret;
}

int mul(int a, int b) {
    long long ret = 1L * a * b;
    return ret % DIV;
}

int main() {
    int n;
    cin >> n;
    vector<int> v(n);
    for (int i = 0; i < n; ++i) {
        cin >> v[i];
    }
    if (v[0] != 0) {
        cout << 0 << endl;
        return 0;
    }
    map<int, int> m;
    for (int num : v) ++m[num];

    int answer = 1;
    int prev = 1;
    for (int i = 0; i < m.end()->first; ++i) {
        int b = m[i];
        if (i == 0) {
            if (b > 1) {
                cout << 0 << endl;
                return 0;
            }
        } else {
            if (b == 0) {
                cout << 0 << endl;
                return 0;
            }
            answer = mul(answer, pow(prev, b));
            prev = b;
        }
    }
    cout << answer << endl;
}
