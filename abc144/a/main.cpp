#include <iostream>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    cout << (a < 10 && b < 10 ? a * b : -1) << endl;
}
