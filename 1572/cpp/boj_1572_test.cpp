// https://www.acmicpc.net/problem/1572

#include <iostream>
#include <vector>

#define dbg(x) (printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, x), x)

#define dbg$(x) { \
    static int _dbg_value = (x); \
    printf("[%s:%d] %s = %d\n", __FILE__, __LINE__, #x, _dbg_value); \
    _dbg_value; \
}

using namespace std;
using ll = long long;


void seg_update(
    vector<int>& seg_tree,
    int node,
    int start,
    int end,
    int idx,
    int diff
) {
    // 범위를 벗어난 경우
    if (idx < start || end < idx)
        return;

    // 리프 노드인 경우
    if (start == end) {
        seg_tree[node] += diff;
        return;
    }

    // 그 외의 경우는 재탐색
    int mid = (start + end) / 2;
    seg_update(seg_tree, node * 2, start, mid, idx, diff);
    seg_update(seg_tree, node * 2 + 1, mid + 1, end, idx, diff);

    // 자식 노드의 값이 변경되었으므로 현재 노드의 값을 갱신
    seg_tree[node] = seg_tree[node * 2] + seg_tree[node * 2 + 1];
}

int seg_query(
    const vector<int>& seg_tree,
    int node,
    int start,
    int end,
    int index
) {
    // cout << "node: " << node << ", start: " << start << ", end: " << end << ", index: " << index << endl;
    if (start == end)
        return start;

    int mid = (start + end) / 2;
    if (index <= seg_tree[node * 2])
        return seg_query(seg_tree, node * 2, start, mid, index);
    return seg_query(seg_tree, node * 2 + 1, mid + 1, end, index - seg_tree[node * 2]);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    // 입력값의 범위
    constexpr int MAX = 65536 + 1;

    int n, k;
    cin >> n >> k;

    vector<int> arr(n + 1);

    // 이후에 들어올 값의 크기는 0 ~ 65535이므로 seg_tree의 크기를 65536 * 4로 설정
    vector<int> seg_tree(MAX * 4, 0);

    int mid_index = (k + 1) / 2;
    for (int i = 0; i < k-1; i++)
    {
        cin >> arr[i];

        // seg_tree의 arr[i]번째 인덱스에 1을 더함
        seg_update(seg_tree, 1, 0, n + 1, arr[i], 1);
    }


    ll result = 0;
    for (int i = k-1; i < n; i++)
    {
        cin >> arr[i];

        // 입력된 값을 seg_tree에 추가
        seg_update(seg_tree, 1, 0, n + 1, arr[i], 1);

        // 중앙값을 구하고 결과에 더함
        result += seg_query(seg_tree, 1, 0, n + 1, mid_index);

        // 맨 앞에 있는 값을 seg_tree에서 제거
        seg_update(seg_tree, 1, 0, n + 1, arr[i - k + 1], -1);
    }

    cout << result << '\n';

    return 0;
}

void test() {
    vector<int> arr = { 1, 2, 3, 4 };
    vector<int> seg_tree(4 * 4, 0);

    auto print = [&seg_tree]() {
        cout << "seg_tree: ";
        for (const auto& i : seg_tree)
            cout << i << ' ';
        cout << endl;
    };

    seg_update(seg_tree, 1, 0, 4, 1, 1);
    print();
    dbg$(seg_query(seg_tree, 1, 0, 4, 1));
    cout << endl;

    seg_update(seg_tree, 1, 0, 4, 2, 1);
    print();
    dbg$(seg_query(seg_tree, 1, 0, 4, 2));
    cout << endl;

    seg_update(seg_tree, 1, 0, 4, 3, 1);
    print();
    dbg$(seg_query(seg_tree, 1, 0, 4, 3));
    cout << endl;

    seg_update(seg_tree, 1, 0, 4, 4, 1);
    print();
    dbg$(seg_query(seg_tree, 1, 0, 4, 4));
    cout << endl;
    dbg$(seg_query(seg_tree, 1, 0, 4, 3));
    cout << endl;
}