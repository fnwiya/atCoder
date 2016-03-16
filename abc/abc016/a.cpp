#include <iostream>
using namespace std;
int M, D;

int main() {
  cin >> M >> D;
  if (M % D == 0) {
    cout << "YES" << endl;
  } else {
    cout << "NO" << endl;
  }
}
