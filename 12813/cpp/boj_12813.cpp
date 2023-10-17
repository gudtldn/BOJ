#include <iostream>
#include <string>

using namespace std;

class BinaryVector
{
private:
    bool binary_array[100'000];

public:
    BinaryVector() : binary_array{0} {}
    BinaryVector(const string& binary_string)
    {
        for (int i = 0; i < 100'000; i++)
        {
            binary_array[99'999 - i] = binary_string[99'999 - i] - 48;
        }
    }

    friend std::ostream& operator<<(std::ostream& os, const BinaryVector& obj) {
        for (int i = 0; i < 100'000; i++)
        {
            os << obj.binary_array[i];
        }
        return os;
    }

    BinaryVector operator&(const BinaryVector& obj)
    {
        BinaryVector result;
        for (int i = 0; i < 100'000; i++)
        {
            result.binary_array[i] = binary_array[i] & obj.binary_array[i];
        }
        return result;
    }

    BinaryVector operator|(const BinaryVector& obj)
    {
        BinaryVector result;
        for (int i = 0; i < 100'000; i++)
        {
            result.binary_array[i] = binary_array[i] | obj.binary_array[i];
        }
        return result;
    }

    BinaryVector operator^(const BinaryVector& obj)
    {
        BinaryVector result;
        for (int i = 0; i < 100'000; i++)
        {
            result.binary_array[i] = binary_array[i] ^ obj.binary_array[i];
        }
        return result;
    }

    BinaryVector operator~()
    {
        BinaryVector result;
        for (int i = 0; i < 100'000; i++)
        {
            result.binary_array[i] = !binary_array[i];
        }
        return result;
    }
};

int main()
{
    string a, b;
    cin >> a >> b;

    BinaryVector binary_vector_a{a};
    BinaryVector binary_vector_b{b};

    cout << (binary_vector_a & binary_vector_b) << '\n';
    cout << (binary_vector_a | binary_vector_b) << '\n';
    cout << (binary_vector_a ^ binary_vector_b) << '\n';
    cout << (~binary_vector_a) << '\n';
    cout << (~binary_vector_b) << '\n';
}