import sys
from io import StringIO
from unittest import TestCase, main
from boj_11723 import solution

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
            Test.run_solution("26\nadd 1\nadd 2\ncheck 1\ncheck 2\ncheck 3\nremove 2\ncheck 1\ncheck 2\ntoggle 3\ncheck 1\ncheck 2\ncheck 3\ncheck 4\nall\ncheck 10\ncheck 20\ntoggle 10\nremove 20\ncheck 10\ncheck 20\nempty\ncheck 1\ntoggle 1\ncheck 1\ntoggle 1\ncheck 1"),
            "1\n1\n0\n1\n0\n1\n0\n1\n0\n1\n1\n0\n0\n0\n1\n0"
        )


if __name__ == "__main__":
    main()
