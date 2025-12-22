from random import choice, randint
from typing import Final
from time import sleep

DATA: Final[str] = "POCKYCHOCOLATE"
e2e: dict[str, list[str]] = {}
for i in range(0, len(DATA) - 1):
    if DATA[i] in e2e:
        e2e[DATA[i]].append(DATA[i + 1])
    else:
        e2e[DATA[i]] = [DATA[i + 1]]


def generate() -> str:
    current: str = choice(DATA)
    print(current, end="")
    while True:
        if randint(0, len(DATA)):
            if not current[-1] in e2e:
                break
            candidates: list[str] = e2e[current[-1]]
            if not candidates:
                break
            next: str = choice(candidates)
            print(next, end="")
            current += next
        else:
            break
    print()
    return current


while True:
    generate()
    sleep(1)
