
class AsmCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm",
            gdb.COMMAND_USER,
            gdb.COMPLETE_NONE,
            prefix=True,
        )

class PrintCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm print",
            gdb.COMMAND_USER,
            gdb.COMPLETE_NONE,
            prefix=True,
        )

class PrintStackCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm print stack",
            gdb.COMMAND_DATA,
            gdb.COMPLETE_NONE,
        )

    def invoke(self, argument, from_tty):
        sp = gdb.parse_and_eval("vm.registers.sp")
        count = min(1000000 - sp, 10)
        res = gdb.execute(f"print *(vm.memory.buf.ptr.pointer.pointer+vm.registers.sp-vm.code.len-256)@{count}", to_string=True)
        print(res)

class NextCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm next",
            gdb.COMMAND_USER,
            gdb.COMPLETE_NONE,
            prefix=False,
        )

    def invoke(self, argument, from_tty):
        gdb.execute("continue")

AsmCommand()
PrintCommand()
PrintStackCommand()
NextCommand()

bp = gdb.Breakpoint("stack_assembly_interpreter::logic::vm::Executor<stack_assembly_interpreter::logic::stdio::Stdio>::execute_step<stack_assembly_interpreter::logic::stdio::Stdio>")
gdb.execute("run")
