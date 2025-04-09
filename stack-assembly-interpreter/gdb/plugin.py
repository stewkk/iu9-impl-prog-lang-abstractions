
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

def print_stack():
    sp = gdb.parse_and_eval("vm.registers.sp")
    count = min(1000000 - sp, 10)
    if count < 1:
        return
    res = gdb.parse_and_eval(f"*(vm.memory.buf.ptr.pointer.pointer+vm.registers.sp-vm.code.len-256)@{count}")
    print(res.format_string(pretty_arrays=True))

class PrintStackCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm print stack",
            gdb.COMMAND_DATA,
            gdb.COMPLETE_NONE,
        )

    def invoke(self, argument, from_tty):
        print_stack()

def print_registers():
    registers = gdb.parse_and_eval("vm.registers")
    print(registers.format_string(pretty_structs=True))


class PrintRegistersCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm print registers",
            gdb.COMMAND_DATA,
            gdb.COMPLETE_NONE,
        )

    def invoke(self, argument, from_tty):
        print_registers()

def print_current_command():
    try:
        ip = gdb.parse_and_eval("vm.registers.ip")
        instruction = gdb.parse_and_eval(f"vm.code.buf.ptr.pointer.pointer[{ip-256}].token")
        try:
            print(instruction['Integer']['__0'])
        except:
            pass
        try:
            ptr = instruction['Ident']['__0']['vec']['buf']['ptr']['pointer']['pointer']
            sz = instruction['Ident']['__0']['vec']['len']
            gdb.execute(f"printf \"%.{sz}s\\n\", {ptr}")
        except:
            pass
    except:
        pass

class PrintInstructionCommand(gdb.Command):
    def __init__(self):
        super().__init__(
            "asm print instruction",
            gdb.COMMAND_DATA,
            gdb.COMPLETE_NONE,
        )

    def invoke(self, argument, from_tty):
        print_current_command()

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
        try:
            print_current_command()
            print_stack()
            print_registers()
        except:
            pass

AsmCommand()

PrintCommand()
PrintStackCommand()
PrintRegistersCommand()
PrintInstructionCommand()

NextCommand()

bp = gdb.Breakpoint("stack_assembly_interpreter::logic::vm::Executor<stack_assembly_interpreter::logic::stdio::Stdio>::execute_step<stack_assembly_interpreter::logic::stdio::Stdio>")
gdb.execute("run")
print_current_command()
print_stack()
