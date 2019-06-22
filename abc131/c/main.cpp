#include <iostream>

using namespace std;

long long gcd(long long a, long long b) {
    return a == 0 ? b : gcd(b % a, a);
}

int main() {
    long long a, b, c, d;
    cin >> a >> b >> c >> d;
    long long answer = b - a + 1;
    answer -= (b / c) - (a / c);
    if (a % c == 0) --answer;
    answer -= (b / d) - (a / d);
    if (a % d == 0) --answer;
    long long g = gcd(c, d);
    long long e = g * (c / g) * (d / g);
    answer += (b / e) - (a / e);
    if (a % e == 0) ++answer;
    cout << answer << endl;
}
