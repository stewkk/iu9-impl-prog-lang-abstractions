#!/usr/bin/env python3

import pytest
import subprocess

def test_example():
    process = subprocess.run(
        [
            "refal/refgo",
            "task1.rsl",
        ],
        capture_output=True,
        text=True,
    )
    output = process.stdout.rstrip()
    assert output == "Hello"
