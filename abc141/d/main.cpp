#include <iostream>
#include <queue>

using namespace std;

int main() {
    int n, m;
    long long answer = 0;
    cin >> n >> m;
    priority_queue<int> q;
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        q.push(a);
        answer += a;
    }
    for (int i = 0; i < m; ++i) {
        int t = q.top();
        q.pop();
        answer -= t / 2;
        if (t % 2 != 0) --answer;
        q.push(t / 2);
    }
    cout << answer << endl;
}
