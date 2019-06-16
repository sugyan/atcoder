#include <iostream>

using namespace std;

int main() {
    int w, h, x, y;
    cin >> w >> h >> x >> y;
    double answer = w / 2.0 * h;
    int z = 0;
    if (w == x * 2 && h == y * 2) {
        z = 1;
    }
    cout << answer << " " << z << endl;
}
