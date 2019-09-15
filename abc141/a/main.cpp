#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    if (s == "Sunny") {
        cout << "Cloudy" << endl;
    }
    if (s == "Cloudy") {
        cout << "Rainy" << endl;
    }
    if (s == "Rainy") {
        cout << "Sunny" << endl;
    }
}
