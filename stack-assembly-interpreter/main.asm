;; "123\n"
;; ... -> ... '1' '2' '3'
:ReadLine
    SETFP
    :Loop
        IN
        10
        SUB
        Loop
        JNE
        GETRV
        RET
