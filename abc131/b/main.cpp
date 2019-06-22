#include <iostream>

using namespace std;

int main() {
    int n, l;
    cin >> n >> l;
    int answer = n * (n - 1) / 2 + n * l;
    if (l > 0) {
        answer -= l;
    } else if (l + n <= 0) {
        answer -= l + n - 1;
    }
    cout << answer << endl;
}
