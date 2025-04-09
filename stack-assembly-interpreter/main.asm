ReadLine
CALL

Stoi
CALL

GETRV
OUT

0
HALT

;;; NOTE: read line from stdin and save to :String as null terminated string
:ReadLine
    SETFP
    String
    LOAD                        ; p
    :LoopReadLine
        IN                      ; p '1'
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

:String
2000
