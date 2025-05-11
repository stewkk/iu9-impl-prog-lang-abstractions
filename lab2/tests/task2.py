#!/usr/bin/env python3

#!/usr/bin/env python3

import pytest
import subprocess

TMP = "/tmp/refal_test_task2.txt"


@pytest.mark.parametrize("stdin, expected_stdout",
                         [
                             ("""(((((a "*" b) "*" c) "+" (d "*" e)) "+" (f "*" g)) "*" (h "+" i))""",
                              "((((h * (c * (a * b )))+ (i * (c * (a * b ))))+ ((h * (e * d ))+ (i * (e * d ))))+ ((h * (g * f ))+ (i * (g * f ))))\n"),
                             ("""(((((a "*" b) "*" c) "+" (d "*" e)) "+" (f "*" g)) "*" h)""",
                              "((((a * b )* (c * h ))+ (d * (e * h )))+ (f * (g * h )))\n"),
                             ("""((a "/" b) "/" (c "/" d))""",
                              "((a * d )/ (c * b ))\n")
                         ])
def test_task1(stdin, expected_stdout):
    with open(TMP, 'w') as f: f.write(stdin)

    process = subprocess.run(
        [
            "refal/refgo",
            "task2.rsl+LibraryEx.rsl",
            TMP,
        ],
        capture_output=True,
        text=True,
    )
    output = process.stdout, process.stderr

    assert output == (expected_stdout, "")
