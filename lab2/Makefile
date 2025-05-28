args = ""
args2 = "tests/data/test_task2_input.txt"

all: compile
	@echo "------------------------------" && refal/refgo task1.rsl $(args)

2: compile
	@echo "------------------------------" && refal/refgo task2.rsl+LibraryEx.rsl $(args2)

test: compile
	@pytest -vvv tests/*.py

compile:
	@refal/refc task1.ref
	@refal/refc task2.ref

clean:
	@rm task1.rsl
	@rm task2.rsl


.PHONY: all 2 clean compile
