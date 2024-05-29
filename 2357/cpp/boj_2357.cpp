#include <functional>
#include <iostream>
#include <vector>

using namespace std;
using ll = long long;

ll seg_init(
    const vector<ll>& arr,
    vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    function<ll(ll, ll)> func
) {
    if (start == end) // 리프 노드인 경우
        return seg_tree[node] = arr[start];

    size_t mid = (start + end) / 2;
    return seg_tree[node] = func(seg_init(arr, seg_tree, node * 2, start, mid, func),
                                 seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end, func));
}

ll max_query(
    const vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start) // 구간 밖에 있는 경우
        return 0;

    if (left <= start && end <= right) // 구간 안에 있는 경우
        return seg_tree[node];

    size_t mid = (start + end) / 2;
    return max(max_query(seg_tree, node * 2, start, mid, left, right),
               max_query(seg_tree, node * 2 + 1, mid + 1, end, left, right));
}

ll min_query(
    const vector<ll>& seg_tree,
    size_t node,
    size_t start,
    size_t end,
    size_t left,
    size_t right
) {
    if (left > end || right < start) // 구간 밖에 있는 경우
        return 1e9;

    if (left <= start && end <= right) // 구간 안에 있는 경우
        return seg_tree[node];

    size_t mid = (start + end) / 2;
    return min(min_query(seg_tree, node * 2, start, mid, left, right),
               min_query(seg_tree, node * 2 + 1, mid + 1, end, left, right));
}

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    size_t n, m;
    cin >> n >> m;

    vector<ll> arr(n);
    vector<ll> seg_min_tree(n * 4);
    vector<ll> seg_max_tree(n * 4);
    for (auto& x : arr)
        cin >> x;
    seg_init(arr, seg_min_tree, 1, 0, n - 1, [](ll a, ll b) { return min(a, b); });
    seg_init(arr, seg_max_tree, 1, 0, n - 1, [](ll a, ll b) { return max(a, b); });

    for (size_t i = 0; i < m; i++)
    {
        size_t a, b;
        cin >> a >> b;

        cout << min_query(seg_min_tree, 1, 0, n - 1, a - 1, b - 1) << ' '
             << max_query(seg_max_tree, 1, 0, n - 1, a - 1, b - 1) << '\n';
    }
}
