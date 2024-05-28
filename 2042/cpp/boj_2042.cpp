#include <iostream>
#include <vector>

using namespace std;
using ll = long long;


/**
 * @brief 세그먼트 트리를 초기화하는 함수
 *
 * @param arr 값이 저장된 배열
 * @param seg_tree 세그먼트 트리로 사용할 배열
 * @param node 노드 번호
 * @param start 시작 인덱스
 * @param end 끝 인덱스
 * @return long long 세그먼트 트리의 노드 값
 */
ll seg_init(vector<ll>& arr, vector<ll>& seg_tree, int node, int start, int end)
{
    if (start == end) // 리프 노드인 경우
        return seg_tree[node] = arr[start];

    // 부모 노드의 값을 자식 노드의 값의 합으로 초기화
    int mid = (start + end) / 2;
    return seg_tree[node] = seg_init(arr, seg_tree, node * 2, start, mid)
                          + seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end);
}

/**
 * @brief 세그먼트 트리의 특정 인덱스의 값을 변경하는 함수
 *
 * @param seg_tree 세그먼트 트리
 * @param node 노드 번호
 * @param start 시작 인덱스
 * @param end 끝 인덱스
 * @param index 바꿀 인덱스
 * @param diff 바꿀 값의 차
 */
void seg_update(vector<ll>& seg_tree, int node, int start, int end, int index, ll diff)
{
    if (start > index || index > end) // 범위 밖에 있는 경우
        return;

    seg_tree[node] += diff;

    if (start != end) // 리프 노드가 아닌 경우
    {
        int mid = (start + end) / 2;
        seg_update(seg_tree, node * 2, start, mid, index, diff); // 왼쪽 자식 노드
        seg_update(seg_tree, node * 2 + 1, mid + 1, end, index, diff); // 오른쪽 자식 노드
    }
}

/**
 * @brief 세그먼트 트리에서 특정 구간의 합을 구하는 함수
 * 
 * @param seg_tree 세그먼트 트리
 * @param node 노드 번호
 * @param start 시작 인덱스
 * @param end 끝 인덱스
 * @param left 더할 구간의 시작
 * @param right 더할 구간의 끝
 * @return long long 구간의 합
 */
ll seg_sum(const vector<ll>& seg_tree, int node, int start, int end, int left, int right)
{
    // 범위 밖에 있는 경우
    // 구간 합을 구하고자 하는 범위가 아닌 경우
    if (left > end || right < start)
        return 0;

    // 구간 합을 구하고자 하는 범위에 포함되는 경우
    if (left <= start && end <= right)
        return seg_tree[node];

    // 그 외의 경우는 재탐색
    int mid = (start + end) / 2;
    return seg_sum(seg_tree, node * 2, start, mid, left, right)
         + seg_sum(seg_tree, node * 2 + 1, mid + 1, end, left, right);
}

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    int n, m, k;
    cin >> n >> m >> k;

    vector<ll> arr(n);
    vector<ll> seg_tree(n * 4);
    for (auto& x : arr)
        cin >> x;
    seg_init(arr, seg_tree, 1, 0, n-1);

    m += k;
    for (int i = 0; i < m; i++)
    {
        int a, b;
        ll c;
        cin >> a >> b >> c;

        switch (a)
        {
        case 1:
        {
            ll diff = c - arr[b - 1]; // 바뀐 값과 기존 값의 차
            arr[b - 1] = c; // 리프노드 값 바꾸기

            // 트리 업데이트
            seg_update(seg_tree, 1, 0, n-1, b-1, diff);
            break;
        }
        case 2:
        {
            cout << seg_sum(seg_tree, 1, 0, n-1, b-1, c-1) << '\n';
            break;
        }
        }
    }
}
