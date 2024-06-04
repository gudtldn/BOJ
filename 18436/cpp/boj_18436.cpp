// https://www.acmicpc.net/problem/18436

#include <iostream>
#include <vector>
#include <functional>

#define dbg$(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, (x)), x)

using namespace std;
using ll = long long;

void seg_init(
    const vector<ll>& arr,
    vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end,
    bool is_even = true
) {
    if (start == end) {
        if (is_even) {
            seg[node] = (arr[start] % 2 == 0) ? 1 : 0;
        } else {
            seg[node] = (arr[start] % 2 == 1) ? 1 : 0;
        }
        return;
    }

    size_t mid = (start + end) / 2;
    seg_init(arr, seg, node * 2, start, mid);
    seg_init(arr, seg, node * 2 + 1, mid + 1, end);
    seg[node] = seg[node * 2] + seg[node * 2 + 1];

}

void seg_update(
    vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end,
    size_t index,
    ll value,
    bool is_even = true
) {
    if (index < start || index > end) {
        return;
    }

    if (start == end) {
        if (is_even) {
            seg[node] = (value % 2 == 0) ? 1 : 0;
        } else {
            seg[node] = (value % 2 == 1) ? 1 : 0;
        }
        return;
    }

    size_t mid = (start + end) / 2;
    seg_update(seg, node * 2, start, mid, index, value, is_even);
    seg_update(seg, node * 2 + 1, mid + 1, end, index, value, is_even);
    seg[node] = seg[node * 2] + seg[node * 2 + 1];
}

ll seg_query(
    const vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start) {
        return 0;
    }

    if (left <= start && end <= right) {
        return seg[node];
    }

    size_t mid = (start + end) / 2;
    return seg_query(seg, node * 2, start, mid, left, right)
         + seg_query(seg, node * 2 + 1, mid + 1, end, left, right);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, m;
    cin >> n;

    vector<ll> arr(n);
    vector<ll> seg_even(n * 4);
    vector<ll> seg_odd(n * 4);

    for (auto& x : arr) {
        cin >> x;
    }

    seg_init(arr, seg_even, 1, 0, n - 1);
    seg_init(arr, seg_odd, 1, 0, n - 1, false);

    cin >> m;
    while(m--)
    {
        int cmd, a, b;
        cin >> cmd >> a >> b;

        switch (cmd)
        {
        case 1:
        {
            seg_update(seg_even, 1, 0, n - 1, a - 1, b);
            seg_update(seg_odd, 1, 0, n - 1, a - 1, b, false);
            break;
        }
        case 2:
        {
            cout << seg_query(seg_even, 1, 0, n - 1, a - 1, b - 1) << '\n';
            break;
        }
        case 3:
        {
            cout << (b - a + 1) - seg_query(seg_even, 1, 0, n - 1, a - 1, b - 1) << '\n';
            break;
        }
        }
    }

    return 0;
}
