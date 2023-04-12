from enum import Enum, auto
from os import mkdir, listdir, chdir, getcwd, system, startfile


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

    @classmethod
    def all(cls) -> list[Enum]:
        return [*cls]


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
                    open(f"./boj_{n}.py", "w", encoding="utf-8").close()

                case Languages.Rust:
                    system(f"cargo new boj_{n}")
                
                case Languages.Cpp:
                    open(f"./boj_{n}.cpp", "w", encoding="utf-8").close()

    startfile(f"{getcwd()}\\{n}")


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


if __name__ == "__main__":
    main()
