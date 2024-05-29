#include <iostream>
#include <vector>

using namespace std;
using ll = long long;

ll seg_init(const vector<ll>& arr, vector<ll>& seg_tree, size_t node, size_t start, size_t end)
{
    if (start == end)
        return seg_tree[node] = arr[start];

    size_t mid = (start + end) / 2;
    return seg_tree[node] = min(seg_init(arr, seg_tree, node * 2, start, mid),
                                seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end));
}

ll min_query(const vector<ll>& seg_tree, size_t node, size_t start, size_t end, size_t left, size_t right)
{
    if (left > end || right < start)
        return 1e9;

    if (left <= start && end <= right)
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
    vector<ll> seg_tree(n * 4);
    for (auto& x : arr)
        cin >> x;

    seg_init(arr, seg_tree, 1, 0, n - 1);

    for (size_t i{}; i < m; i++)
    {
        size_t a, b;
        cin >> a >> b;

        cout << min_query(seg_tree, 1, 0, n - 1, a - 1, b - 1) << '\n';
    }
}