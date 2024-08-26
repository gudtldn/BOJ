import sys
from io import StringIO
from unittest import TestCase, main
from boj_1037 import solution

class Test(TestCase):
    @staticmethod
    def run_solution(input_str: str) -> str:
        with (
            StringIO(input_str) as fake_input,
            StringIO() as fake_output
        ):
            original_output = sys.stdout
            sys.stdout = fake_output
            solution(iter(fake_input.read().splitlines()).__next__)
            sys.stdout = original_output
            return fake_output.getvalue().strip()

    def test_solution1(self) -> None:
        self.assertEqual(Test.run_solution("2\n4 2"), "8")

    def test_solution2(self) -> None:
        self.assertEqual(Test.run_solution("1\n2"), "4")

    def test_solution3(self) -> None:
        self.assertEqual(Test.run_solution("6\n3 4 2 12 6 8"), "24")

    def test_solution4(self) -> None:
        self.assertEqual(Test.run_solution("14\n14 26456 2 28 13228 3307 7 23149 8 6614 46298 56 4 92596"), "185192")

if __name__ == "__main__":
    main()
