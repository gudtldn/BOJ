#include <iostream>
#include <string>

using namespace std;

int main()
{
    int t;
    cin >> t;

    for (int i = 0; i < t; i++)
    {
        int n;
        string s;
        
        cin >> n >> s;
        s.erase(n-1, 1);
        cout << s << "\n";
    }
    
    return 0;
}