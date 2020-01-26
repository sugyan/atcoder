#include <iostream>

using namespace std;

int main() {
    int h, n;
    int sum = 0;
    cin >> h >> n;
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        sum += a;
    }
    cout << (sum >= h ? "Yes" : "No") << endl;
}
