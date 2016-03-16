#include <iostream>
using namespace std;
int A, B, C;
bool is_plus, is_minus;

int main()
{
  cin >> A >> B >> C;
  if (A + B == C) {
    is_plus = true;
  }
  if (A - B == C) {
    is_minus = true;
  }
  if (is_plus && is_minus) {
    cout << "?" << endl;
  } else if (is_plus){
    cout << "+" << endl;
  } else if (is_minus) {
    cout << "-" << endl;
  } else {
    cout << "!" << endl;
  }
}
