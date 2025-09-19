import re
import textwrap
from os import mkdir, listdir, chdir, getcwd, system
from enum import Enum, auto
from datetime import timedelta
from dataclasses import dataclass

from requests import get
from bs4 import BeautifulSoup


@dataclass(frozen=True)
class ProblemExample:
    input: str
    output: str

@dataclass(frozen=True)
class Problem:
    problem_id: int                 # 문제 번호
    title: str                      # 문제 제목
    time_limit: timedelta           # 제한 시간
    time_limit_detail: str | None   # 언어별 제한 시간 상세

    memory_limit: int               # 제한 메모리 (MB)
    memory_limit_detail: str | None # 언어별 제한 메모리 상세

    desc: str                       # 문제 설명
    input_desc: str                 # 입력 설명
    output_desc: str                # 출력 설명
    constraints: str                # 제약 사항

    examples: list[ProblemExample]  # 예제 입력/출력 쌍
    hints: str                      # 힌트

    source: str                     # 출처


class ChangeDir():
    def __init__(self, new_path: str) -> None:
        self.default_path = getcwd()
        self.new_path = new_path

    def __enter__(self):
        chdir(self.new_path)

    def __exit__(self, *args):
        chdir(self.default_path)


class Languages(Enum):
    Cpp = auto()
    Python = auto()
    Rust = auto()
    Kotlin = auto()
    x86_64_Assembly = auto()

    @classmethod
    def all(cls) -> list[Enum]:
        return [*cls]


def get_problem(problem_id: int) -> Problem:
    """
    문제 ID를 받아 해당 문제의 정보를 크롤링하고 파싱하여 Problem 객체를 반환합니다.
    """
    url = f"https://www.acmicpc.net/problem/{problem_id}"
    response = get(
        url,
        headers={
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36",
            "Referer": "https://www.acmicpc.net/",
        }
    )

    try:
        response.raise_for_status()
        soup = BeautifulSoup(response.text, "lxml")

        title: str = soup.select_one("#problem_title").text.strip()

        info_table = soup.select_one("#problem-info")
        time_limit_str: str = info_table.select_one("tr:nth-child(1) > td:nth-child(1)").text
        memory_limit_str: str = info_table.select_one("tr:nth-child(1) > td:nth-child(2)").text

        time_limit_sec = int(re.search(r"(\d+)", time_limit_str).group(1))
        time_limit = timedelta(seconds=time_limit_sec)

        specific_time_limit_elem = soup.select_one("#problem-time-limit > ul")
        time_limit_detail = None
        if specific_time_limit_elem:
            time_limit_detail = specific_time_limit_elem.get_text(separator="\n", strip=True)
        
        memory_limit = int(re.search(r"(\d+)", memory_limit_str).group(1))

        # 언어별 메모리 제한이 있는 경우, memory_limit_str을 덮어쓴다.
        specific_memory_limit_elem = soup.select_one("#problem-memory-limit > ul")
        memory_limit_detail = None
        if specific_memory_limit_elem:
            memory_limit_detail = specific_memory_limit_elem.get_text(separator="\n", strip=True)

        desc = soup.select_one("#problem_description").get_text(separator="\n", strip=True)
        input_desc = soup.select_one("#problem_input").get_text(separator="\n", strip=True)
        output_desc = soup.select_one("#problem_output").get_text(separator="\n", strip=True)
        
        constraints_elem = soup.select_one("#problem_limit")
        constraints = constraints_elem.get_text(separator="\n", strip=True) if constraints_elem else ""

        example_inputs = soup.select('pre[id^="sample-input-"]')
        example_outputs = soup.select('pre[id^="sample-output-"]')

        examples = []
        for i in range(len(example_inputs)):
            input_text = example_inputs[i].get_text(strip=True).replace('\r\n', '\n').replace('\r', '\n')
            output_text = example_outputs[i].get_text(strip=True).replace('\r\n', '\n').replace('\r', '\n')
            examples.append(ProblemExample(input_text, output_text))

        hints_elem = soup.select_one("#problem_hint")
        hints = hints_elem.get_text(separator="\n", strip=True) if hints_elem else ""

        source_ul = soup.select_one("#source > ul")
        if source_ul:
            source_lines = [li.get_text(strip=True) for li in source_ul.select("li")]
            source = "\n".join(source_lines)
        else:
            source = ""

        return Problem(
            problem_id=problem_id,
            title=title,
            time_limit=time_limit,
            time_limit_detail=time_limit_detail,
            memory_limit=memory_limit,
            memory_limit_detail=memory_limit_detail,
            desc=desc,
            input_desc=input_desc,
            output_desc=output_desc,
            constraints=constraints,
            examples=examples,
            hints=hints,
            source=source,
        )
    except Exception as e:
        print(f"문제 정보를 가져오는 중 오류가 발생했습니다: {e}")
        raise

def question_number() -> str:
    while True:
        n = input("문제 번호를 입력해 주세요: ")
        if n.isnumeric():
            return n

        print("다시 입력해 주세요.\n")


def selected_langs() -> list[Languages]:
    print(*[f"{lang.value}. {lang.name}" for lang in Languages.all()], sep="\n")
    while True:
        langs = list(input("언어를 선택해 주세요: ").replace(" ", ""))

        if all(lang.isnumeric() for lang in langs):
            return [lang for lang in Languages.all() if lang.value in map(int, langs)]

        print("다시 입력해 주세요.\n")


def main():
    n = question_number()
    print()
    langs = selected_langs()
    print()

    problem = get_problem(int(n))

    if n not in listdir("./"):
        mkdir(f"./{n}")

    for lang in langs:
        if lang.name.lower() in listdir(f"./{n}"):
            print(f"이미 {lang.name.lower()} 폴더가 존재합니다.")
            continue

        mkdir(f"./{n}/{lang.name.lower()}")
        with ChangeDir(f"./{n}/{lang.name.lower()}"):
            match lang:
                case Languages.Python:
                    if ".vscode" not in listdir("../"):
                        mkdir("../.vscode")

                    with open("../.vscode/settings.json", "w", encoding="utf-8") as settings:
                        settings.write(textwrap.dedent("""
                            {
                                "python.testing.unittestArgs": [
                                    "-v",
                                    "-s",
                                    "./python",
                                    "-p",
                                    "*_test.py"
                                ],
                                "python.testing.pytestEnabled": false,
                                "python.testing.unittestEnabled": true
                            }
                        """).lstrip())

                    with (
                        open(f"./boj_{n}.py", "w", encoding="utf-8") as py,
                        open(f"./boj_{n}_test.py", "w", encoding="utf-8") as test_py
                    ):
                        # 아래와 같이 test코드와 호환해서 사용하면, 속도가 좀 느려짐
                        py.write(textwrap.dedent(f"""
                            # {problem.title}
                            # https://www.acmicpc.net/problem/{n}

                            from typing import Callable

                            def solution(it_next: Callable[[], str]):
                                ...

                            if __name__ == "__main__":
                                solution(iter(open(0).read().splitlines()).__next__)
                        """).lstrip())

                        tc_template = """
                                def test_solution{n}(self) -> None:
                                    self.assertEqual(
                                        Test.run_solution("{input_str}"),
                                        "{output_str}"
                                    )
                        """

                        test_cases: list[str] = []
                        for i, example in enumerate(problem.examples, start=1):
                            test_cases.append(
                                tc_template.format(
                                    n=i,
                                    input_str=example.input.replace('"""', r'\"\"\"').replace("\n", "\\n"),
                                    output_str=example.output.replace('"""', r'\"\"\"').replace("\n", "\\n")
                                )
                            )

                        test_py.write(textwrap.dedent(f"""
                            import sys
                            from io import StringIO
                            from unittest import TestCase, main
                            from boj_{n} import solution

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
                                {''.join(test_cases)}

                            if __name__ == "__main__":
                                main()
                        """).lstrip())

                case Languages.Rust:
                    # lib.rs 파일을 만들어서 테스트 코드를 작성하면 좋을 것 같음
                    system(f"cargo new boj_{n} --vcs none")
                    with (
                        open("../Cargo.toml", "w", encoding="utf-8") as root_toml,
                        open(f"./boj_{n}/src/main.rs", "w", encoding="utf-8") as rs
                    ):
                        root_toml.write(textwrap.dedent(f"""
                            [workspace]
                            members = ["rust/boj_{n}"]
                            resolver = "2"
                        """).lstrip())

                        tc_template = """
                                #[test]
                                fn test_solution{n}() {{
                                    assert_eq!(
                                        capture_output(|| solution("{input_str}")),
                                        "{output_str}"
                                    );
                                }}
                        """

                        test_cases: list[str] = []
                        for i, example in enumerate(problem.examples, start=1):
                            test_cases.append(
                                tc_template.format(
                                    n=i,
                                    input_str=example.input.replace('"""', r'\"\"\"').replace("\n", "\\n"),
                                    output_str=example.output.replace('"""', r'\"\"\"').replace("\n", "\\n")
                                )
                            )

                        rs.write(textwrap.dedent(f"""
                            // {problem.title}
                            // https://www.acmicpc.net/problem/{n}

                            fn solution(stdin: &str) {{
                                let mut tokens = stdin.split('\\n');
                                let mut next = || tokens.next().unwrap();

                                #[allow(unused_macros)]
                                macro_rules! parse_line {{
                                    ($($to_type:ty),*) => {{
                                        {{
                                            let mut iter = next().split_whitespace();
                                            ($(iter.next().unwrap().parse::<$to_type>().unwrap()),*)
                                        }}
                                    }};
                                }}

                                todo!("solve here");
                            }}


                            #[cfg(test)]
                            mod tests {{
                                use super::*;
                                fn capture_output(f: impl FnOnce()) -> String {{
                                    STDOUT.with(|stdout| {{
                                        stdout.borrow_mut().clear();
                                    }});
                                    f();
                                    STDOUT.with(|stdout| {{
                                        String::from_utf8(stdout.borrow().to_vec())
                                            .unwrap()
                                            .trim_end()
                                            .to_string()
                                    }})
                                }}
                                {''.join(test_cases)}
                            }}

                            fn main() {{
                                use std::io::*;
                                solution(&read_to_string(stdin()).unwrap());
                                STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
                            }}

                            thread_local! {{
                                #[cfg(test)]
                                static STDOUT: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());

                                #[cfg(not(test))]
                                static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> =
                                    std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
                            }}

                            #[macro_export]
                            macro_rules! println {{
                                ($($t:tt)*) => {{
                                    STDOUT.with(|refcell| {{
                                        use std::io::*;
                                        writeln!(refcell.borrow_mut(), $($t)*).unwrap();
                                    }})
                                }};
                            }}

                            #[macro_export]
                            macro_rules! print {{
                                ($($t:tt)*) => {{
                                    STDOUT.with(|refcell| {{
                                        use std::io::*;
                                        write!(refcell.borrow_mut(), $($t)*).unwrap();
                                    }})
                                }};
                            }}
                        """).lstrip())

                case Languages.Cpp:
                    with (
                        open(f"./boj_{n}.cpp", "w", encoding="utf-8") as cpp,
                        open("./run.bat", "w", encoding="utf-8") as run_bat,
                        open("./debug.bat", "w", encoding="utf-8") as debug_bat,
                        open("./debug.gdb", "w", encoding="utf-8") as debug_gdb
                    ):
                        cpp.write(textwrap.dedent(f"""
                            // {problem.title}
                            // https://www.acmicpc.net/problem/{n}

                            #include <iostream>


                            void solution()
                            {{
                                using namespace std;

                                // code here
                            }}

                            int main()
                            {{
                                std::cin.tie(nullptr);
                                std::cout.tie(nullptr);
                                std::ios::sync_with_stdio(false);

                                solution();

                                return 0;
                            }}
                        """).lstrip())
                        run_bat.write(textwrap.dedent(f"""
                            @echo off
                            g++ -std=c++20 -o boj_{n} boj_{n}.cpp
                            IF %ERRORLEVEL% NEQ 0 EXIT
                            boj_{n}.exe
                        """).lstrip())
                        debug_gdb.write("r\nq\n")
                        debug_bat.write(textwrap.dedent(f"""
                            @echo off
                            g++ -std=c++20 -g -o boj_{n} boj_{n}.cpp
                            IF %ERRORLEVEL% NEQ 0 EXIT
                            gdb -q -x debug.gdb boj_{n}.exe
                        """).lstrip())

                case Languages.Kotlin:
                    with (
                        open(f"./boj_{n}.kt", "w", encoding="utf-8") as kt,
                        open("./run.bat", "w", encoding="utf-8") as run_bat,
                    ):
                        kt.write(textwrap.dedent(f"""
                            // {problem.title}
                            // https://www.acmicpc.net/problem/{n}

                            fun main() {{
                                // code here
                            }}
                        """).lstrip())
                        run_bat.write(textwrap.dedent(f"""
                            @echo off
                            kotlinc boj_{n}.kt -include-runtime -d boj_{n}.jar
                            IF %ERRORLEVEL% NEQ 0 EXIT
                            java -jar boj_{n}.jar
                        """).lstrip())

                case Languages.x86_64_Assembly:
                    with open(f"./boj_{n}.asm", "w", encoding="utf-8") as asm:
                        asm.write(textwrap.dedent(f"""
                            ; {problem.title}
                            ; https://www.acmicpc.net/problem/{n}

                            section .data
                                ; data here

                            section .bss
                                ; bss here

                            section .text
                                global main
                                extern printf
                                extern scanf

                            main:
                                push rbp
                                mov rbp, rsp

                                ; code here

                                pop rbp
                                xor rax, rax
                                ret
                        """).lstrip())

    system(f"code {getcwd()}\\{n}")

if __name__ == "__main__":
    main()
