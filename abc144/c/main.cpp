#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

using namespace std;

int main() {
    long long n;
    cin >> n;
    vector<long long> v;
    for (long long i = 1; i <= sqrt(n); ++i) {
        if (n % i == 0) {
            v.push_back(i);
            if (i * i != n) {
                v.push_back(n / i);
            }
        }
    }
    long long answer = n - 1;
    for (long long l : v) {
        answer = min(answer, l + (n / l) - 2);
    }
    cout << answer << endl;
}
