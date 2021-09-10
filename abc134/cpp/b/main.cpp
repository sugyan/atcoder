#include <iostream>

using namespace std;

int main() {
    int n, d;
    cin >> n >> d;
    int answer = n / (d * 2 + 1);
    if (n % (d * 2 + 1) != 0) ++answer;
    cout << answer << endl;
}
