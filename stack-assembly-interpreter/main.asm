ReadLine
CALL

Stoi
CALL

ToHex
CALL
;;; NOTE: stack = [std::end(String)]

OutputHexReversed
CALL

0
HALT

;;; NOTE: read line from stdin and save to :String as null terminated string
:ReadLine
    SETFP
    String
    LOAD                        ; p
    IN                          ; p '1'
    DUP                         ; p '1' '1'
    45                          ; p '1' '1' '-'
    SUB                         ; p '1' ('1' - '-')
    PositiveNumber
    JNE

    IsNeg
    LOAD                        ; p '-' IsNeg
    1
    SAVE
    DROP
    IN
    LoopReadLine
    JMP
    :PositiveNumber
    IsNeg
    LOAD                        ; p '1' IsNeg
    0
    SAVE
    :LoopReadLine
        DUP                     ; p '1' '1'
        10
        SUB                     ; p '1' '1'-'\n'
        EndLoopReadLine
        JEQ
        OVER                    ; p '1' p
        SWAP                    ; p p '1'
        SAVE                    ; p ; *p = '1'
        1
        ADD                     ; p++
        IN                      ; p '1'
        LoopReadLine
        JMP
    :EndLoopReadLine
    DROP
    0
    SAVE
    GETFP
    RET

;;; rv = stoi(*String)
:Stoi
    SETFP

    String
    LOAD                        ; p
    LOAD                        ; '1'
    DUP
    SkipEarlyReturn
    JNE
    0
    HALT
    :SkipEarlyReturn
    48
    SUB                         ; 1

    String                      ; 1 p
    LOAD
    :LoopStoi
        1
        ADD                     ; 1 p+1
        DUP                     ; 1 p+1 p+1
        LOAD                    ; 1 p+1 '2'
        DUP                     ; 1 p+1 '2' '2'
        LoopStoiEnd
        JEQ
        48
        SUB                     ; 1 p+1 2
        ROT                     ; p+1 2 1
        10
        MUL                     ; p+1 2 10
        ADD                     ; p+1 12
        SWAP                    ; 12 p+1
        LoopStoi
        JMP
    :LoopStoiEnd                ; 1 p 0
    DROP2
    SETRV
    GETFP
    RET

:ToHex
    SETFP

    String
    LOAD                        ; p
    :LoopToHex
    GETRV                       ; p 123
    DUP                         ; p 123 123
    ToHexRet                    ;
    JEQ                         ; p 0
    16
    MOD                         ; p 123%16

    DUP                         ; p digit digit
    10
    SUB                         ; p digit digit - 10
    GreaterThanNineToHex
    JGE
    ;; if 123%16 < 10
    48
    ADD                         ; p '0'
    EndIfToHex
    JMP
    :GreaterThanNineToHex
    ;; p digit
    10
    SUB                         ; digit - 10
    65
    ADD                         ; p 'A'
    :EndIfToHex
    SWAP                        ; 'A' p
    DUP                         ; 'A' p p
    ROT                         ; p p 'A'
    SAVE                        ; p
    1
    ADD                         ; p+1
    GETRV                       ; p+1 123
    16
    DIV                         ; p+1 123/16
    SETRV                       ; p+1
    LoopToHex
    JMP

    :ToHexRet                   ; p 0
    DROP                        ; p
    GETFP
    RET

:OutputHexReversed              ; std::end(String)
    SETFP

    IsNeg
    LOAD
    LOAD
    SkipOutputMinus
    JEQ
    45
    OUT
    :SkipOutputMinus
    48
    OUT
    120
    OUT

    DUP                         ; it it
    String
    LOAD
    SUB                         ; it it-String
    ZeroOutputHexReversedRet
    JEQ

    :LoopOutputHexReversed
    1
    SUB                         ; it
    String
    LOAD                        ; it 2000
    SWAP                        ; 2000 it
    DUP                         ; 2000 it it
    ROT                         ; it it 2000
    SUB                         ; it it-2000
    OutputHexReversedRet
    JLT
    ;;; it
    DUP
    LOAD                        ; it 'A'
    OUT                         ; it
    LoopOutputHexReversed
    JMP

    :ZeroOutputHexReversedRet
    48
    OUT

    :OutputHexReversedRet
    GETFP
    RET

:String
2000
:IsNeg
3000
