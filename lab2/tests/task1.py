#!/usr/bin/env python3

import pytest
import subprocess
import shlex


@pytest.mark.parametrize("args, expected_stdout, expected_stderr",
                         [
                             ("test_single.txt", "test_single.txt", ""),
                         ])
def test_task1(args, expected_stdout, expected_stderr):
    process = subprocess.run(
        [
            "refal/refgo",
            "task1.rsl",
        ]+shlex.split(args),
        capture_output=True,
        text=True,
    )

    output = process.stdout.rstrip(), process.stderr.rstrip()

    assert output == (expected_stdout, expected_stderr)
