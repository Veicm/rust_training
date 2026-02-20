def count_letters(file_path: str) -> dict[str, int]:
    with open(file_path, "r") as txt_file:
        lorem: str = txt_file.read()

    letters: list[str] = list(lorem)
    result: dict[str, int] = {}

    for letter in letters:
        if result.get(letter):
            result[letter] += 1
        else:
            result[letter] = 1

    result_sorted = {
        k: v for k, v in sorted(result.items(), key=lambda item: item[1], reverse=True)
    }
    return result_sorted


def main() -> None:
    print(count_letters("lorem.txt"))


if __name__ == "__main__":
    main()
