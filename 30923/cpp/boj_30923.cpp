#include <iostream>

using namespace std;

int main()
{
    // 첫째 줄에 히스토그램을 이루는 막대의 개수를 의미하는 정수 N이 주어진다. (1 <= N <= 32,768)
    int n;
    cin >> n;

    // 둘째 줄에 각 막대의 높이를 의미하는 정수 h_1, h_2, ..., h_N이 공백으로 구분되어 주어진다. (1 <= h_i <= 32,768)
    int h[n];
    for (int i = 0; i < n; i++)
    {
        cin >> h[i];
    }

    // 계산
    long long sum_h = 0, // 막대의 높이의 합
              side_h = 0; // 막대 사이의 높이 차의 합

    for (int i = 0; i < n; i++)
    {
        sum_h += h[i]; // 입력된 막대의 면적

        if (i > 0)
        {
            side_h += abs(h[i - 1] - h[i]); // 막대 사이의 높이 차
        }
    }
    side_h += h[0] + h[n - 1]; // 처음과 마지막 막대의 높이

    cout << ((n + sum_h) * 2 + side_h) << endl;
}
