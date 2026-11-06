#!/usr/bin/env python3
"""Historic script. The `harmonic` CLI mode is still a stub and writes no CSV."""

from __future__ import annotations

import sys

from harmonic_csv import reject_stub_csv


def main() -> None:
    reject_stub_csv("harmonic")


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as exc:
        print(exc, file=sys.stderr)
        sys.exit(2)
