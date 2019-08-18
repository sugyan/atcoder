#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

void dfs(int target, unordered_map<int, vector<int>>& um, int parent, vector<int>& answer) {
    for (int child : um[target]) {
        if (child != parent) {
            answer[child] += answer[target];
            dfs(child, um, target, answer);
        }
    }
}

int main() {
    int n, q;
    cin >> n >> q;
    unordered_map<int, vector<int>> um;
    for (int i = 0; i < n - 1; ++i) {
        int a, b;
        cin >> a >> b;
        um[a - 1].push_back(b - 1);
        um[b - 1].push_back(a - 1);
    }
    vector<int> answer(n, 0);
    for (int i = 0; i < q; ++i) {
        int p, x;
        cin >> p >> x;
        answer[p - 1] += x;
    }
    dfs(0, um, -1, answer);
    for (int i = 0; i < n; ++i) {
        cout << answer[i];
        if (i < n - 1) cout << " ";
    }
    cout << endl;
}
