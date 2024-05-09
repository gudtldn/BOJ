#include <iostream>
#include <string>
#include <optional>

using namespace std;

template <typename T>
class Queue
{
private:
    T *data;
    int front_idx;
    int back_idx;
    int capacity;

public:
    Queue(int capacity) : front_idx(0), back_idx(0), capacity(capacity)
    {
        data = new T[capacity];
    }

    ~Queue()
    {
        delete[] data;
    }

    void push(T value)
    {
        data[back_idx] = value;
        back_idx = (back_idx + 1) % capacity;
    }

    T pop()
    {
        if (empty())
        {
            return -1;
        }

        T value = data[front_idx];
        front_idx = (front_idx + 1) % capacity;
        return value;
    }

    int size()
    {
        return (back_idx - front_idx + capacity) % capacity;
    }

    int empty()
    {
        return front_idx == back_idx;
    }

    T front()
    {
        if (empty())
        {
            return -1;
        }

        return data[front_idx];
    }

    T back()
    {
        if (empty())
        {
            return -1;
        }

        return data[(back_idx - 1 + capacity) % capacity];
    }
};

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    Queue<int> queue(10000);

    int n;
    cin >> n;

    while (n--)
    {
        string cmd;
        cin >> cmd;

        if (cmd == "push")
        {
            int arg;
            cin >> arg;
            queue.push(arg);
        }
        else if (cmd == "pop")
        {
            cout << queue.pop() << '\n';
        }
        else if (cmd == "size")
        {
            cout << queue.size() << '\n';
        }
        else if (cmd == "empty")
        {
            cout << queue.empty() << '\n';
        }
        else if (cmd == "front")
        {
            cout << queue.front() << '\n';
        }
        else if (cmd == "back")
        {
            cout << queue.back() << '\n';
        }
    }

    return 0;
}