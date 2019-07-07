#include <iostream>
#include <cmath>
#include <vector>

using namespace std;

int main() {
    int n, d;
    cin >> n >> d;
    vector<vector<int>> x(n, vector<int>(d));
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < d; ++j) {
            cin >> x[i][j];
        }
    }
    int answer = 0;
    for (int i = 0; i < n - 1; ++i) {
        for (int j = i + 1; j < n; ++j) {
            int sqsum = 0;
            for (int k = 0; k < d; ++k) {
                sqsum += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            int sq = sqrt(sqsum);
            if (sq * sq == sqsum) ++answer;
        }
    }
    cout << answer << endl;
}
