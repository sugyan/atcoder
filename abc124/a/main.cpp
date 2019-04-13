#include <bits/stdc++.h>

using namespace std;

int main() {
    int a, b, answer;
    cin >> a >> b;
    answer = max(a, b) * 2;
    if (a != b) --answer;
    cout << answer << endl;
}
