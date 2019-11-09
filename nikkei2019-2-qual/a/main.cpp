#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    int answer = n / 2;
    if (n % 2 == 0) --answer;
    cout << answer << endl;
}
