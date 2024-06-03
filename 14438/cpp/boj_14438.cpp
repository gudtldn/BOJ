// https://www.acmicpc.net/problem/14438
#include <iostream>
#include <vector>

#define dbg$(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, (x)), x)

using namespace std;
using ll = long long;

ll seg_init(
    const vector<ll>& arr,
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end
) {
    if (start == end) {
        return seg_tree[node] = arr[start];
    }

    size_t mid = (start + end) / 2;
    return seg_tree[node] = min(
        seg_init(arr, seg_tree, node * 2, start, mid),
        seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end)
    );
}

ll seg_query(
    const vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start) {
        return 1e9;
    }

    if (left <= start && end <= right) {
        return seg_tree[node];
    }

    size_t mid = (start + end) / 2;
    return min(
        seg_query(seg_tree, node * 2, start, mid, left, right),
        seg_query(seg_tree, node * 2 + 1, mid + 1, end, left, right)
    );
}

void seg_update(
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t index,
    ll value
) {
    if (index < start || index > end) {
        return;
    }

    seg_tree[node] = value;

    if (start != end)
    {
        size_t mid = (start + end) / 2;
        seg_update(seg_tree, node * 2, start, mid, index, value);
        seg_update(seg_tree, node * 2 + 1, mid + 1, end, index, value);

        seg_tree[node] = min(seg_tree[node * 2], seg_tree[node * 2 + 1]);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, m;
    cin >> n;

    vector<ll> arr(n);
    vector<ll> seg_tree(n * 4);

    for (auto& x : arr) {
        cin >> x;
    }

    seg_init(arr, seg_tree, 1, 0, n - 1);

    cin >> m;
    while (m--) {
        int cmd, a, b;
        cin >> cmd >> a >> b;
        if (cmd == 1) {
            seg_update(seg_tree, 1, 0, n - 1, a - 1, b);
        } else {
            cout << seg_query(seg_tree, 1, 0, n - 1, a - 1, b - 1) << '\n';
        }
    }

    return 0;
}
