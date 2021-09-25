#include <iostream>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    if ((a + b) % 2 != 0) {
        cout << "IMPOSSIBLE" << endl;
    } else {
        cout << (a + b) / 2 << endl;
    }
}
