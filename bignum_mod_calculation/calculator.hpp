#ifndef CALCULATOR_HPP
#define CALCULATOR_HPP

class CalcSet {
    private:
        int number;
        int exponential;
        int mod;
    public:
        CalcSet();
        void GetNum();
        int calculation();
};

#endif