#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

struct Person {
    int age;
    string name;
    int order;
};

int main() {
    int n;
    cin >> n;
    vector<Person> people(n);
    for (int i = 0; i < n; i++) {
        cin >> people[i].age >> people[i].name;
        people[i].order = i;
    }

    sort(
        people.begin(),
        people.end(),
        [](const Person &a, const Person &b) -> bool
    {
        if (a.age == b.age)
            return a.order < b.order;
        return a.age < b.age;
    });

    for (int i = 0; i < n; i++) {
        cout << people[i].age << ' ' << people[i].name << '\n';
    }
    return 0;
}