import json
import random
import subprocess
from os import path
from typing import List

import matplotlib.pyplot as plt
from tqdm import tqdm

BINARY = path.abspath("./target/release/wordle")


def run_subprocess(answer):
    try:
        # Run subprocess and capture stdout
        result = subprocess.run(
            [BINARY, "--answer", answer],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            stdin=subprocess.PIPE,
            text=True,
            check=True,
        )

        lines_count = len(result.stdout.splitlines())

        return lines_count
    except subprocess.CalledProcessError:
        # Handle subprocess errors if needed
        return 7


def main():
    with open("./answers.json", "r") as f:
        answers: List[str] = json.loads(f.read())

    random.shuffle(answers)

    amount = int(input("How many words should we test? (max 2315):  "))

    results = []
    for i in tqdm(answers[:amount]):
        results.append(run_subprocess(i))

    print("average:", sum(results) / len(results))


if __name__ == "__main__":
    main()
