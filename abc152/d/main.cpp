#include <iostream>

using namespace std;

int count(int i, int j, int n) {
    int ret = 0;
    int m = n;
    int k = 0;
    while (m >= 10) {
        m /= 10;
        ++k;
    }
    if (k > 1) {
        if (m == i) {
            int l = 1;
            while (k-- > 0) l *= 10;
            ret += (n - i * l) / 10;
            if (n % 10 >= j) ++ret;
        } else if (m > i) {
            int l = 1;
            while (--k > 0) l *= 10;
            ret += l;
        }
    } else {
        if (n >= i * 10 + j) ++ret;
    }
    if (i == j && n >= i) ++ret;
    if (n > 99999) ret += 1000;
    if (n > 9999) ret += 100;
    if (n > 999) ret += 10;
    if (n > 99) ret += 1;
    return ret;
}

int main() {
    int n;
    cin >> n;
    int answer = 0;
    for (int i = 1; i < 10; ++i) {
        for (int j = 1; j < 10; ++j) {
            answer += count(i, j, n) * count(j, i, n);
        }
    }
    cout << answer << endl;
}
