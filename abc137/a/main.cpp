#include <iostream>

using namespace std;

int main() {
    int a, b;
    cin >> a >> b;
    int answer = a + b;
    answer = max(answer, a - b);
    answer = max(answer, a * b);
    cout << answer << endl;
}
