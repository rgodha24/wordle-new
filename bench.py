import asyncio
import json
import random
from os import path
from sys import argv
from typing import List

from tqdm.asyncio import tqdm as async_tqdm

BINARY = path.abspath("./target/release/wordle")


async def run_subprocess(answer):
    try:
        # Run subprocess and capture stdout
        process = await asyncio.create_subprocess_exec(
            BINARY, "--answer", answer,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

        stdout, stderr = await process.communicate()

        if process.returncode != 0:
            # Handle subprocess errors if needed
            return 7

        lines_count = len(stdout.decode().splitlines())

        return lines_count
    except Exception as e:
        print(f"An error occurred: {e}")
        return 7


async def main():
    with open("./answers.json", "r") as f:
        answers: List[str] = json.loads(f.read())

    random.shuffle(answers)

    # get answer out of argv
    try:
        amount = int(argv[1])
    except:
        print("no amount passed in, defaulting to 100")
        amount = 100

    tasks = [run_subprocess(answer) for answer in answers[:amount]]
    results = []
    for f in async_tqdm(asyncio.as_completed(tasks), total=len(tasks)):
        result = await f
        results.append(result)

    print("average:", sum(results) / len(results))


if __name__ == "__main__":
    asyncio.run(main())