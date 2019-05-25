#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    int n1 = stoi(s.substr(0, 2)), n2 = stoi(s.substr(2, 2));
    if (0 < n1 && n1 <= 12) {
        if (0 < n2 && n2 <= 12) {
            cout << "AMBIGUOUS";
        } else {
            cout << "MMYY";
        }
    } else if (0 < n2 && n2 <= 12) {
        cout << "YYMM";
    } else {
        cout << "NA";
    }
    cout << endl;
}
