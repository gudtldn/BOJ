import textwrap
from enum import Enum, auto
from os import mkdir, listdir, chdir, getcwd, system


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


def question_number() -> str:
    while True:
        n = input("문제 번호를 입력해 주세요: ")
        if n.isnumeric():
            return n

        print("다시 입력해 주세요.\n")


def selected_langs() -> list[Languages]:
    print(*[f"{n}. {lang}" for n, lang in [(l.value, l.name) for l in Languages.all()]], sep="\n")
    while True:
        lang = list(input("언어를 선택해 주세요: ").replace(" ", ""))

        if all(l.isnumeric() for l in lang):
            return [l for l in Languages.all() if l.value in map(int, lang)]

        print("다시 입력해 주세요.\n")


def main():
    n = question_number()
    print()
    langs = selected_langs()
    print()

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
                            # https://www.acmicpc.net/problem/{n}
                            from typing import Callable
                            
                            def solution(it_next: Callable[[], str]):
                                ...

                            if __name__ == "__main__":
                                solution(iter(open(0).read().splitlines()).__next__)
                        """).lstrip())
                        test_py.write(textwrap.dedent(f"""
                            import sys
                            from io import StringIO
                            from unittest import TestCase, main
                            from boj_{n} import solution

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
                                    self.assertEqual(Test.run_solution(...), ...)

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
                        rs.write(textwrap.dedent(f"""
                            // https://www.acmicpc.net/problem/{n}

                            fn solution(stdin: &str) {{
                                let mut tokens = stdin.split('\\n');
                                let mut next = || tokens.next().unwrap();

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

                                #[test]
                                fn test_solution1() {{
                                    assert_eq!(capture_output(|| solution(...)), ...);
                                }}
                            }}

                            fn main() {{
                                use std::io::*;
                                solution(&read_to_string(stdin()).unwrap());
                                STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
                            }}

                            #[cfg(test)]
                            thread_local! {{
                                static STDOUT: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());
                            }}

                            #[cfg(not(test))]
                            thread_local! {{
                                static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> =
                                std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
                            }}

                            #[macro_export]
                            macro_rules! println {{
                                ($($t:tt)*) => {{
                                    STDOUT.with(|refcell| {{
                                        use std::io::*;
                                        writeln!(refcell.borrow_mut(), $($t)*).unwrap();
                                    }});
                                }};
                            }}

                            #[macro_export]
                            macro_rules! print {{
                                ($($t:tt)*) => {{
                                    STDOUT.with(|refcell| {{
                                        use std::io::*;
                                        write!(refcell.borrow_mut(), $($t)*).unwrap();
                                    }});
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
                            // https://www.acmicpc.net/problem/{n}
                            #include <iostream>
                            #define FASTIO std::cin.tie(nullptr); std::cout.tie(nullptr); std::ios::sync_with_stdio(false);

                            int main()
                            {{
                                using namespace std;
                                FASTIO

                                // code here

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
