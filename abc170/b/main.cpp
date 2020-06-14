#include <iostream>

using namespace std;

int main() {
    int x, y;
    cin >> x >> y;
    cout << (y >= 2 * x && y <= 4 * x && y % 2 == 0 ? "Yes" : "No") << endl;
}
