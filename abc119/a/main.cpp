#include <iostream>
#include <cstdlib>

using namespace std;

int main() {
    string s;
    cin >> s;
    int n = atoi(s.substr(5, 2).c_str());
    if (n <= 4) {
        cout << "Heisei" << endl;
    } else {
        cout << "TBD" << endl;
    }
}