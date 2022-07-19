#include<iostream>
#include<vector>
#include<algorithm>
#include "calculator.hpp"
using namespace std;


CalcSet::CalcSet() {
    number = 9726;
    exponential = 3533;
    mod = 11413;
}

void CalcSet::GetNum() {
    cout<<"a^m = b (mod n) の形の式における b を求めます．\n" ;

    cout<<"根となる数値aを入力してください: ";
    cin>>number;

    cout<<"べき乗数mを入力してください: ";
    cin>>exponential;

    cout<<"法nを入力してください．: ";
    cin>>mod;
}

void CalcSet::GetNum(int num, int exp, int m) {
    number = num;
    exponential = exp;
    mod = m;
}

int CalcSet::calculation() {
    long long int buff = exponential;
    vector<int> binary;

    while(buff != 1) {
        binary.push_back(buff % 2);
        buff = buff / 2;
    }
    binary.push_back(1);

    reverse(begin(binary), end(binary));

    buff = 1;
    for (const auto& i: binary){
        if (i == 1){
            buff = buff * buff * number % mod;
        }else{
            buff = buff * buff % mod;
        }
    }

    return buff;
}