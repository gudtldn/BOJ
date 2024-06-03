// https://www.acmicpc.net/problem/11505
#include <iostream>
#include <vector>

#define dbg$(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, (x)), x)

using namespace std;
using ll = long long;


ll seg_init(
    const vector<ll>& arr,
    vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end
) {
    if (start == end)
        return seg[node] = arr[start];
    
    size_t mid = (start + end) / 2;
    return seg[node] = (
        seg_init(arr, seg, node * 2, start, mid)
        * seg_init(arr, seg, node * 2 + 1, mid + 1, end)
    ) % 1000000007;
}

ll seg_query(
    const vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start)
        return 1;
    if (left <= start && end <= right)
        return seg[node];
    
    size_t mid = (start + end) / 2;
    return (
        seg_query(seg, node * 2, start, mid, left, right)
        * seg_query(seg, node * 2 + 1, mid + 1, end, left, right)
    ) % 1000000007;
}

void seg_update(
    vector<ll>& seg,
    size_t node,
    size_t start,
    size_t end,
    size_t index,
    ll value
) {
    if (index < start || index > end)
        return;

    seg[node] = value;

    if (start != end)
    {
        size_t mid = (start + end) / 2;
        seg_update(seg, node * 2, start, mid, index, value);
        seg_update(seg, node * 2 + 1, mid + 1, end, index, value);
        seg[node] = (seg[node * 2] * seg[node * 2 + 1]) % 1000000007;
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, m, k;
    cin >> n >> m >> k;

    vector<ll> arr(n);
    vector<ll> seg_tree(n * 4);    

    for (auto& a : arr)
        cin >> a;

    seg_init(arr, seg_tree, 1, 0, n - 1);

    m += k;
    while (m--)
    {
        size_t a, b, c;
        cin >> a >> b >> c;

        if (a == 1)
        {
            arr[b - 1] = c;
            seg_update(seg_tree, 1, 0, n - 1, b - 1, c);
        }
        else
        {
            cout << seg_query(seg_tree, 1, 0, n - 1, b - 1, c - 1) << '\n';
        }
    }

    return 0;
}
