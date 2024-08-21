from boj_1009 import solution
from io import StringIO
from unittest import TestCase, main

class Test(TestCase):
    def test_solution1(self) -> None:
        buffer = StringIO("5\n1 6\n3 7\n6 2\n7 100\n9 635")
        self.assertEqual(solution(buffer), "1\n7\n6\n1\n9")

    def test_solution2(self) -> None:
        buffer = StringIO("1\n1 1")
        self.assertEqual(solution(buffer), "1")

    def test_solution3(self) -> None:
        buffer = StringIO("1\n2 2")
        self.assertEqual(solution(buffer), "4")

    def test_solution4(self) -> None:
        buffer = StringIO("1\n99 999999")
        self.assertEqual(solution(buffer), "9")

if __name__ == "__main__":
    main()

