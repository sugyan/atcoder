#include <iostream>
#include <vector>
#include <sstream>
#include <iterator>
#include <unordered_map>

using namespace std;

int main() {
    int n;
    cin >> n;
    unordered_map<string, long long> um;
    for (int i = 0; i < n; ++i) {
        vector<int> d(26, 0);
        string s;
        cin >> s;
        for (char c : s) ++d[c - 'a'];
        ostringstream oss;
        copy(d.begin(), d.end(), ostream_iterator<long long>(oss, "-"));
        ++um[oss.str()];
    }
    long long answer = 0;
    for (pair<string, long long> p : um) {
        if (p.second > 1) {
            answer += (p.second - 1) * (p.second) / 2;
        }
    }
    cout << answer << endl;
}
