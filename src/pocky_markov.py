from random import choice, randint
from typing import Final
from time import sleep

DATA: Final[str] = "POCKYCHOCOLATE"
v2v: dict[str, list[str]] = {}
for i in range(0, len(DATA) - 1):
    if DATA[i] in v2v:
        v2v[DATA[i]].append(DATA[i + 1])
    else:
        v2v[DATA[i]] = [DATA[i + 1]]


def generate() -> str:
    current: str = choice(DATA)
    print(current, end="")
    while True:
        if randint(0, len(DATA)):
            if not current[-1] in v2v:
                break
            candidates: list[str] = v2v[current[-1]]
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
