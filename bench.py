import json
import random
import subprocess
from os import path
from sys import argv
from typing import List

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

    # get answer out of argv
    try:
        amount = int(argv[1])
    except:
        print("no amount passed in, defaulting to 100")
        amount = 100

    results = []
    pbar = tqdm(total = amount)
    for i in answers[:amount]:
        pbar.write(f"running {i}")
        results.append(run_subprocess(i))
        pbar.update(1)

    print("average:", sum(results) / len(results))


if __name__ == "__main__":
    main()
