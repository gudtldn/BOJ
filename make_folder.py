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
                    with open(f"./boj_{n}.py", "w", encoding="utf-8") as fw:
                        fw.write(f"# https://www.acmicpc.net/problem/{n}\n\n")

                case Languages.Rust:
                    system(f"cargo new boj_{n} --vcs none")

                case Languages.Cpp:
                    with (
                        open(f"./boj_{n}.cpp", "w", encoding="utf-8") as cpp,
                        open("./run.bat", "w", encoding="utf-8") as run_bat,
                        open("./debug.bat", "w", encoding="utf-8") as debug_bat,
                        open("./debug.gdb", "w", encoding="utf-8") as debug_gdb
                    ):
                        cpp.write(
                            f"// https://www.acmicpc.net/problem/{n}\n\n"
                            "#include <iostream>\n\n"
                            "#define dbg$(x) (printf(\"[%s:%d] %s = %d\\n\", __FILE__, __LINE__, #x, (x)), x)\n\n"
                            "using namespace std;\n\n"
                            "int main() {\n"
                            "    ios::sync_with_stdio(false);\n"
                            "    cin.tie(nullptr);\n"
                            "    cout.tie(nullptr);\n\n\n\n"
                            "    return 0;\n"
                            "}\n"
                        )
                        run_bat.write(
                            "@echo off\n"
                            f"g++ -std=c++20 -o boj_{n} boj_{n}.cpp\n"
                            "IF %ERRORLEVEL% NEQ 0 EXIT\n"
                            f"boj_{n}.exe\n"
                        )
                        debug_gdb.write("r\nq\n")
                        debug_bat.write(
                            "@echo off\n"
                            f"g++ -std=c++20 -g -o boj_{n} boj_{n}.cpp\n"
                            "IF %ERRORLEVEL% NEQ 0 EXIT\n"
                            f"gdb -q -x debug.gdb boj_{n}.exe\n"
                        )

                case Languages.Kotlin:
                    with (
                        open(f"./boj_{n}.kt", "w", encoding="utf-8") as kt,
                        open("./run.bat", "w", encoding="utf-8") as run_bat,
                    ):
                        kt.write(
                            f"// https://www.acmicpc.net/problem/{n}\n\n"
                            "fun main() {\n\n"
                            "}\n"
                        )
                        run_bat.write(
                            "@echo off\n"
                            f"kotlinc boj_{n}.kt -include-runtime -d boj_{n}.jar\n"
                            "IF %ERRORLEVEL% NEQ 0 EXIT\n"
                            f"java -jar boj_{n}.jar\n"
                        )

    # startfile(f"{getcwd()}\\{n}")
    system(f"code {getcwd()}\\{n}")


if __name__ == "__main__":
    main()
