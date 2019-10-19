#include <iostream>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    cout << max(a - 2 * b, 0) << endl;
}
