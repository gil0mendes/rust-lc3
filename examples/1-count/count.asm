.ORIG x3000

AND R0, R0, 0     ; clear R0
LOOP              ; label to indicate the start of our loop
ADD R0, R0, 1     ; add 1 to R0 and store the result into R0
ADD R1, R0, -10   ; subtract 10 from R0 and store the result on R1
BRn LOOP

HALT              ; halt the program
.end              ; mark the end of the file
