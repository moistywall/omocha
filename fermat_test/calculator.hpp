// mod計算用のプログラム
#pragma once

class CalcSet {
    private:
        int number;
        int exponential;
        int mod;
    public:
        CalcSet();
        void GetNum();
        void GetNum(int num, int exp, int m);
        int calculation();
};
