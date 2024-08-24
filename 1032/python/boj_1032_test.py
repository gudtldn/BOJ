import sys
from io import StringIO
from unittest import TestCase, main
from boj_1032 import solution

class Test(TestCase):
    @staticmethod
    def run_solution(input_str: str) -> str:
        with (
            StringIO(input_str) as fake_input,
            StringIO() as fake_output
        ):
            original_stdout = sys.stdout
            sys.stdout = fake_output
            solution(iter(fake_input.read().splitlines()).__next__)
            sys.stdout = original_stdout
            return fake_output.getvalue().strip()

    def test_solution1(self) -> None:
        self.assertEqual(Test.run_solution("3\nconfig.sys\nconfig.inf\nconfigures"), "config????")

    def test_solution2(self) -> None:
        self.assertEqual(Test.run_solution("2\ncontest.txt\ncontext.txt"), "conte?t.txt")

    def test_solution3(self) -> None:
        self.assertEqual(Test.run_solution("3\nc.user.mike.programs\nc.user.nike.programs\nc.user.rice.programs"), "c.user.?i?e.programs")

    def test_solution4(self) -> None:
        self.assertEqual(Test.run_solution("4\na\na\nb\nb"), "?")

    def test_solution5(self) -> None:
        self.assertEqual(Test.run_solution("1\nonlyonefile"), "onlyonefile")

if __name__ == "__main__":
    main()
