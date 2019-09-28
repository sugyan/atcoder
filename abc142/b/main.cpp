#include <iostream>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    int answer = 0;
    for (int i = 0; i < n; ++i) {
        int h;
        cin >> h;
        if (h >= k) ++answer;
    }
    cout << answer << endl;
}
