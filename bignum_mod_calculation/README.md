# 大きな数の合同計算が行えるプログラム．

$$a^m \equiv b (mod n)$$

のような合同式が与えられたとき $a^m$ があまりにも大きて計算が難しい場合に用いるプログラム．べき乗根，べき乗数，法の入力がそれぞれ求められるので入力すると自動で計算を行ってくれる．

## 例

$$5761^{6597} \equiv b (mod11413)$$
におけるbを求めたいときにプログラムに従って，べき乗根5761,べき乗数6597，法11413を入力すると下のように計算の途中過程とともに計算結果が出力される．
```
a^m = b (mod n) の形の式における b を求めます．
根となる数値aを入力してください: 5761
べき乗数mを入力してください: 6597
法nを入力してください．: 11413
6597(10) = 1100111000101(2)
1 * 1 * 5761 = 5761 (mod 11413)
5761 * 5761 * 5761 = 670 (mod 11413)
670 * 670 = 3793 (mod 11413)
3793 * 3793 = 6469 (mod 11413)
6469 * 6469 * 5761 = 2726 (mod 11413)
2726 * 2726 * 5761 = 3337 (mod 11413)
3337 * 3337 * 5761 = 7942 (mod 11413)
7942 * 7942 = 7126 (mod 11413)
7126 * 7126 = 3439 (mod 11413)
3439 * 3439 = 2853 (mod 11413)
2853 * 2853 * 5761 = 2500 (mod 11413)
2500 * 2500 = 7089 (mod 11413)
7089 * 7089 * 5761 = 9726 (mod 11413)
計算結果: 9726
```