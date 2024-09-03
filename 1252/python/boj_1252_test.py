import sys
from io import StringIO
from unittest import TestCase, main
from boj_1252 import solution

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
        self.assertEqual(Test.run_solution("1001101 10010"), "1011111")

if __name__ == "__main__":
    main()
