#include <iostream>
#include <set>
#include <vector>

using namespace std;

int a[100005];
int b[100005];
long result[100005];

// https://qiita.com/ofutonfuton/items/c17dfd33fc542c222396
struct UnionFind {
    vector<long> par; // par[i]:iの親の番号　(例) par[3] = 2 : 3の親が2
    vector<long> size;

    UnionFind(long N) : par(N), size(N) { //最初は全てが根であるとして初期化
        for(int i = 0; i < N; i++) {
            par[i] = i;
            size[i] = 1;
        }
    }

    long root(int x) { // データxが属する木の根を再帰で得る：root(x) = {xの木の根}
        if (par[x] == x) return x;
        return par[x] = root(par[x]);
    }

    void unite(int x, int y) { // xとyの木を併合
        int rx = root(x); //xの根をrx
        int ry = root(y); //yの根をry
        if (rx == ry) return; //xとyの根が同じ(=同じ木にある)時はそのまま
        par[rx] = ry; //xとyの根が同じでない(=同じ木にない)時：xの根rxをyの根ryにつける
        size[ry] += size[rx];
    }

    bool same(int x, int y) { // 2つのデータx, yが属する木が同じならtrueを返す
        int rx = root(x);
        int ry = root(y);
        return rx == ry;
    }

    long number(int x) {
        int rx = root(x);
        return size[rx];
    }
};

int main() {
    long n, m;
    cin >> n >> m;
    for (int i = 0; i < m; ++i) {
        cin >> a[i] >> b[i];
    }
    UnionFind tree(n);
    result[0] = n * (n - 1) / 2;
    for (int i = 0; i < m; ++i) {
        long na = tree.number(a[m - 1 - i] - 1);
        long nb = tree.number(b[m - 1 - i] - 1);
        if (tree.same(a[m - 1 - i] - 1, b[m - 1 - i] - 1)) {
            result[i + 1] = result[i];
        } else {
            result[i + 1] = result[i] - na * nb;
        }
        tree.unite(a[m - 1 - i] - 1, b[m - 1 - i] - 1);
    }
    for (int i = m - 1; i >= 0; --i) {
        cout << result[i] << endl;
    }
}