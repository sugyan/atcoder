#include <iostream>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    int n = 1;
    int answer = 0;
    while (n < b) {
        ++answer;
        n += a - 1;
    }
    cout << answer << endl;
}
