#include <iostream>
#include <limits>

using namespace std;

int main() {
    int n, p;
    cin >> n;
    int answer = 0;
    int max = numeric_limits<int>::max();
    for (int i = 0; i < n; ++i) {
        cin >> p;
        if (p < max) {
            max = p;
            ++answer;
        }
    }
    cout << answer << endl;
}
