from boj_1072 import solution
from io import StringIO
from unittest import TestCase, main

class Test(TestCase):
    def test_solution1(self) -> None:
        buffer = StringIO("10 8")
        self.assertEqual(solution(buffer), "1")

    def test_solution2(self) -> None:
        buffer = StringIO("100 80")
        self.assertEqual(solution(buffer), "6")

    def test_solution3(self) -> None:
        buffer = StringIO("47 47 ")
        self.assertEqual(solution(buffer), "-1")

    def test_solution4(self) -> None:
        buffer = StringIO("99000 0")
        self.assertEqual(solution(buffer), "1000")

    def test_solution5(self) -> None:
        buffer = StringIO("1000000000 470000000")
        self.assertEqual(solution(buffer), "19230770")

if __name__ == "__main__":
    main()
