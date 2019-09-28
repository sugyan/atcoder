#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

using namespace std;

long long gcd (long long a, long long b) {
    return b == 0 ? a : gcd(b, a % b);
}

int main() {
    long long a, b;
    cin >> a >> b;
    long long g = gcd(a, b);
    vector<long long> v;
    for (long long i = 1; i <= sqrt(g); ++i) {
        if (g % i == 0) {
            v.push_back(i);
            if (i * i != g) {
                v.push_back(g / i);
            }
        }
    }
    sort(v.begin(), v.end());
    vector<long long> answer;
    for (long long n : v) {
        bool ok = true;
        for (long long m : answer) {
            if (m != 1 && n % m == 0) {
                ok = false;
                break;
            }
        }
        if (ok) {
            answer.push_back(n);
        }
    }
    cout << answer.size() << endl;
}
