import sys
from io import StringIO
from unittest import TestCase, main
from boj_31403 import solution

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
            Test.run_solution("3\n4\n5"),
            "2\n29"
        )


if __name__ == "__main__":
    main()
