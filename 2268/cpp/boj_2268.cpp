// https://www.acmicpc.net/problem/2268

#include <iostream>
#include <vector>

using namespace std;
using ll = long long;


ll seg_query(
    const vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (right < start || end < left)
        return 0;

    if (left <= start && end <= right)
        return seg_tree[node];

    size_t mid = (start + end) / 2;
    return seg_query(seg_tree, node * 2, start, mid, left, right)
         + seg_query(seg_tree, node * 2 + 1, mid + 1, end, left, right);
}

void seg_update(
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t index,
    ll diff
) {
    if (index < start || end < index)
        return;

    seg_tree[node] += diff;

    if (start != end) {
        size_t mid = (start + end) / 2;
        seg_update(seg_tree, node * 2, start, mid, index, diff);
        seg_update(seg_tree, node * 2 + 1, mid + 1, end, index, diff);
    }
}


int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, m;
    cin >> n >> m;

    vector<ll> arr(n + 1, 0);
    vector<ll> seg_tree((n + 1) * 4, 0);

    while (m--)
    {
        size_t cmd, a, b;
        cin >> cmd >> a >> b;

        if (cmd == 0) {
            if (a > b) swap(a, b);
            cout << seg_query(seg_tree, 1, 1, n, a, b) << '\n';
        } else {
            ll diff = b - arr[a];
            arr[a] = b;
            seg_update(seg_tree, 1, 1, n, a, diff);
        }
    }

    return 0;
}
