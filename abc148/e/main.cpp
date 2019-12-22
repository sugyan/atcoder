#include <iostream>

using namespace std;

int main() {
    long long n;
    cin >> n;
    if (n % 2 == 1) {
        cout << 0 << endl;
    } else {
        long long answer = 0;
        long long m = 10;
        while (m <= n) {
            answer += n / m;
            m *= 5;
        }
        cout << answer << endl;
    }
}
