#include <iostream>
#include <cmath>

using namespace std;

const int limit = 1000000;
int N, A, B, C;
int l[8];

int dfs(int n, int a, int b, int c) {
    int result = limit;
    if (n == N) {
        if (a > 0 && b > 0 && c > 0) {
            return abs(A - a) + abs(B - b) + abs(C - c) - 30;
        }
        return result;
    }
    result = min(result, dfs(n + 1, a, b, c));
    result = min(result, dfs(n + 1, a + l[n], b, c) + 10);
    result = min(result, dfs(n + 1, a, b + l[n], c) + 10);
    result = min(result, dfs(n + 1, a , b, c+ l[n]) + 10);
    return result;
}

int main() {
    cin >> N >> A >> B >> C;
    for (int i = 0; i < N; ++i) {
        cin >> l[i];
    }
    cout << dfs(0, 0, 0, 0) << endl;
}