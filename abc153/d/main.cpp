#include <iostream>
#include <unordered_map>

using namespace std;

long long count(unordered_map<long long, long long> &um, long long target) {
    if (target == 1) {
        return 1;
    }
    if (um.find(target) != um.end()) {
        return um[target];
    }
    long long ret = 1 + count(um, target / 2) * 2;
    return um[target] = ret;
}

int main() {
    long long h;
    cin >> h;
    unordered_map<long long, long long> um;
    cout << count(um, h) << endl;
}
