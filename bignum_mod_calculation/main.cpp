#include<iostream>
// #include<vector>
// #include<algorithm>
#include "calculator.hpp"
using namespace std;

// class CalcSet {
//     private:
//         int number;
//         int exponential;
//         int mod;
//     public:
//         CalcSet();
//         void GetNum();
//         int calculation();
// };

// CalcSet::CalcSet() {
//     number = 9726;
//     exponential = 3533;
//     mod = 11413;
// }


// void CalcSet::GetNum() {
//     cout<<"a^m = b (mod n) の形の式における b を求めます．\n" ;

//     cout<<"根となる数値aを入力してください: ";
//     cin>>number;

//     cout<<"べき乗数mを入力してください: ";
//     cin>>exponential;

//     cout<<"法nを入力してください．: ";
//     cin>>mod;
// }

// int CalcSet::calculation() {
//     long long int buff = exponential;
//     vector<int> binary;

//     while(buff != 1) {
//         binary.push_back(buff % 2);
//         buff = buff / 2;
//     }
//     binary.push_back(1);

//     reverse(begin(binary), end(binary));

//     cout<<exponential<<"(10) = ";
//     for (const auto& i: binary){
//         cout<<i;
//     }
//     cout<<"(2)";

//     cout<<endl;

//     buff = 1;
//     for (const auto& i: binary){
//         if (i == 1){
//             cout<<buff<<" * "<<buff<<" * "<<number<<" = ";
//             buff = buff * buff * number % mod;
//             cout<<buff<<" (mod "<<mod<<")"<<endl;
//         }else{
//             cout<<buff<<" * "<<buff<<" = ";
//             buff = buff * buff % mod;
//             cout<<buff<<" (mod "<<mod<<")"<<endl;
//         }
//     }
//     cout<<"計算結果: "<<buff<<endl;

//     return 0;
// }

int main() {
    CalcSet calc;
    calc.GetNum();
    int huga = calc.calculation();
    return 0;
}