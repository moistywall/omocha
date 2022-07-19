#include <iostream>
#include "calculator.hpp"
using namespace std;

void FermatTest(int a, int n) {
    CalcSet test;
    test.GetNum(a, n - 1, n);
    if(test.calculation() == 1){
        cout<< n <<" は " << a <<"を底とした素数です．\n";
    } else {
        cout<< n <<" は素数ではありません．";
    }
}

int main() {
    cout<<"Fermat Test を行います．\n";
    cout<<"底となる数を入力してください : ";
    int a;
    cin >> a;
    cout<<"判定対象とする整数を入力してください : ";
    int n;
    cin >> n;
    FermatTest(a,n);

    return EXIT_SUCCESS;
}