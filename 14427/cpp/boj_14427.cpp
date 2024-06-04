// https://www.acmicpc.net/problem/14427

#include <iostream>
#include <vector>

#define dbg$(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, (x)), x)

using namespace std;
using ll = long long;


void seg_init(
    const vector<ll>& arr,
    vector<pair<ll, size_t>>& seg_tree,
    size_t node,
    size_t start,
    size_t end
) {
    if (start == end)
    {
        seg_tree[node] = make_pair(arr[start], start);
        return;
    }

    size_t mid = (start + end) / 2;
    seg_init(arr, seg_tree, node * 2, start, mid);
    seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end);

    seg_tree[node] = min(seg_tree[node * 2], seg_tree[node * 2 + 1]);
}

void seg_update(
    vector<pair<ll, size_t>>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t index,
    ll value
) {
    if (index < start || index > end)
        return;

    seg_tree[node] = make_pair(value, index);

    if (start != end)
    {
        size_t mid = (start + end) / 2;
        seg_update(seg_tree, node * 2, start, mid, index, value);
        seg_update(seg_tree, node * 2 + 1, mid + 1, end, index, value);

        seg_tree[node] = min(seg_tree[node * 2], seg_tree[node * 2 + 1]);
    }
}

pair<ll, size_t> seg_query(
    const vector<pair<ll, size_t>>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start)
        return make_pair(1e9, 1e9);

    if (left <= start && end <= right)
        return seg_tree[node];

    size_t mid = (start + end) / 2;
    return min(seg_query(seg_tree, node * 2, start, mid, left, right),
               seg_query(seg_tree, node * 2 + 1, mid + 1, end, left, right));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n;
    cin >> n;

    vector<ll> arr(n);
    vector<pair<ll, size_t>> seg_tree(n * 4);

    for (auto& x : arr)
        cin >> x;

    seg_init(arr, seg_tree, 1, 0, n - 1);

    size_t m;
    cin >> m;

    while(m--)
    {
        int cmd;
        cin >> cmd;

        if (cmd == 1)
        {
            size_t i;
            ll v;
            cin >> i >> v;
            seg_update(seg_tree, 1, 0, n - 1, i - 1, v);
        }
        else if (cmd == 2)
        {
            auto min_pair = seg_query(seg_tree, 1, 0, n - 1, 0, n - 1);
            cout << min_pair.second + 1 << '\n';
        }
    }

    return 0;
}
