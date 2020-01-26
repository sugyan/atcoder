#include <iostream>

using namespace std;

int main() {
    int h, a;
    cin >> h >> a;
    cout << (h / a + (h % a == 0 ? 0 : 1)) << endl;
}
