import sys
from io import StringIO
from unittest import TestCase, main
from boj_1036 import solution

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
            Test.run_solution("5\nGOOD\nLUCK\nAND\nHAVE\nFUN\n7"),
            "31YUB"
        )

    def test_solution2(self) -> None:
        self.assertEqual(
            Test.run_solution("1\nHELLO\n2"),
            "ZZLLO"
        )

    def test_solution3(self) -> None:
        self.assertEqual(
            Test.run_solution("5\n500\nPOINTS\nFOR\nTHIS\nPROBLEM\n5"),
            "1100TC85"
        )

    def test_solution4(self) -> None:
        self.assertEqual(
            Test.run_solution("6\nTO\nBE\nOR\nNOT\nTO\nBE\n0"),
            "QNO"
        )

    def test_solution5(self) -> None:
        self.assertEqual(
            Test.run_solution("1\nKEQUALS36\n36"),
            "ZZZZZZZZZ"
        )


if __name__ == "__main__":
    main()
