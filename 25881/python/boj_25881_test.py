import sys
from io import StringIO
from unittest import TestCase, main
from boj_25881 import solution

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
            Test.run_solution("6 10\n4\n1000\n1001\n700\n4800"),
            "1000 6000\n1001 6010\n700 4200\n4800 44000"
        )


if __name__ == "__main__":
    main()
