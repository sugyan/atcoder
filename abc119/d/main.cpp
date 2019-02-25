#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

#define INF 100000000000L

using namespace std;

int main() {
    int a, b, q;
    long x;
    long v;

    cin >> a >> b >> q;
    vector<long> s = { -INF };
    vector<long> t = { -INF };
    for (int i = 0; i < a; ++i) {
        cin >> v;
        s.push_back(v);
    }
    s.push_back(INF);
    for (int i = 0; i < b; ++i) {
        cin >> v;
        t.push_back(v);
    }
    t.push_back(INF);
    for (int i = 0; i < q; ++i) {
        cin >> x;
        long result = INF;
        long s1, s2, t1, t2;
        vector<long>::iterator it;
        it = lower_bound(s.begin(), s.end(), x);
        s1 = *(it - 1);
        s2 = *it;
        it = lower_bound(t.begin(), t.end(), x);
        t1 = *(it - 1);
        t2 = *it;
        result = min(result, abs(s1 - t1) + abs(t1 - x));
        result = min(result, abs(s1 - t1) + abs(s1 - x));
        result = min(result, abs(s2 - t2) + abs(t2 - x));
        result = min(result, abs(s2 - t2) + abs(s2 - x));
        result = min(result, abs(t2 - s1) + abs(t2 - x));
        result = min(result, abs(t2 - s1) + abs(s1 - x));
        result = min(result, abs(t1 - s2) + abs(s2 - x));
        result = min(result, abs(t1 - s2) + abs(t1 - x));
        cout << result << endl;
    }
}