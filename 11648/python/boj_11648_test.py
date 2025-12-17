import sys
from io import StringIO
from unittest import TestCase, main
from boj_11648 import solution

class Test(TestCase):
    @staticmethod
    def run_solution(input_str: str) -> str:
        with (
            StringIO(input_str) as capture_input,
            StringIO() as capture_output
        ):
            original_output = sys.stdout
            sys.stdout = capture_output
            solution(iter(capture_input.read().splitlines()).__next__)
            sys.stdout = original_output
            return capture_output.getvalue().strip()

    def test_solution1(self) -> None:
        self.assertEqual(
            Test.run_solution("5"),
            "0"
        )

    def test_solution2(self) -> None:
        self.assertEqual(
            Test.run_solution("10"),
            "1"
        )

    def test_solution3(self) -> None:
        self.assertEqual(
            Test.run_solution("679"),
            "5"
        )


if __name__ == "__main__":
    main()
