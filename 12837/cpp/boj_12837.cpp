// https://www.acmicpc.net/problem/12837

#include <iostream>
#include <vector>

#define dbg$(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, (x)), x)

using namespace std;
using ll = long long;

void seg_update(
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t idx,
    ll value
) {
    if (idx < start || end < idx) return;

    seg_tree[node] += value;

    if (start != end) {
        size_t mid = (start + end) / 2;
        seg_update(seg_tree, node * 2, start, mid, idx, value);
        seg_update(seg_tree, node * 2 + 1, mid + 1, end, idx, value);

        seg_tree[node] = seg_tree[node * 2] + seg_tree[node * 2 + 1];
    }
}

ll seg_query(
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start)
        return 0;

    if (start >= left && end <= right)
        return seg_tree[node];

    size_t mid = (start + end) / 2;
    return seg_query(seg_tree, node * 2, start, mid, left, right)
         + seg_query(seg_tree, node * 2 + 1, mid + 1, end, left, right);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, q;
    cin >> n >> q;

    vector<ll> seg_tree(n * 4, 0);
    while (q--)
    {
        ll cmd, a, b;
        cin >> cmd >> a >> b;

        if (cmd == 1)
        {
            seg_update(seg_tree, 1, 0, n - 1, a - 1, b);
        }
        else
        {
            cout << seg_query(seg_tree, 1, 0, n - 1, a - 1, b - 1) << '\n';
        }
    }

    return 0;
}
