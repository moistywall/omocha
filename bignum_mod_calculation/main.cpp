#include<iostream>
#include "calculator.hpp"
using namespace std;


int main() {
    CalcSet calc;
    calc.GetNum();
    int huga = calc.calculation();
    return 0;
}