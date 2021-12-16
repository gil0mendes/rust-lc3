.ORIG   x3000

;; Set R0 to 10*R1

AND R2, R2, 0
ADD R2, R2, 48
OUT
        
mul10   ADD     R0, R1, R1      ; R0 ==  2*R1
        ADD     R0, R0, R0      ; R0 ==  4*R1
        ADD     R0, R0, R1      ; R0 ==  5*R1
        ADD     R0, R0, R0      ; R0 == 10*R1

        OUT R0
        HALT
.end
