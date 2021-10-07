;; LC3 Test Code
;;
;; This assembly code is used to test all the opcodes of the LC3 emulator to ensure that everything works great.
;;

  .ORIG x3000

;; ---------- TESTS

;; LEA test
  LEA r0, testlea
  PUTS

  LEA r0, okstring
  PUTS

;; Branch zero test
  LEA r0, testbranchz
  PUTS

  AND r1, r1, 0       ; reset R1
  brz branchzok
  LEA r0, errorstring
  PUTS
  HALT
branchzok
  LEA r0, okstring
  PUTS

;; Branch negative test
  LEA r0, testbranchn
  PUTS

  ADD r0, r1, -10
  brn branchnok
  LEA r0, errorstring
  PUTS
  HALT

branchnok
  LEA r0, okstring
  PUTS

;; Branch positive test
  LEA r0, testbranchp
  PUTS

  AND r1, r1, 0
  ADD r0, r1, 10
  brp branchpok
  LEA r0, errorstring
  PUTS
  HALT

branchpok
  LEA r0, okstring
  PUTS

;; ADD test
  LEA r0, testadd
  PUTS

  AND r1, r1, 0         ; reset r1
  ADD r1, r1, 10
  ADD r0, r1, -10
  brz addtestok
  LEA r0, errorstring
  PUTS
  HALT

addtestok
  LEA r0, okstring
  PUTS

;; ST/LD test
  LEA r0, testst
  PUTS

  AND r1, r1, 0
  ADD r1, r1, 10
  ST r1, var1
  LD r2, var1
  ADD r1, r2, -10
  BRz stldtestok
  LEA r0, errorstring
  PUTS
  HALT

stldtestok
  LEA r0, okstring
  PUTS

;; JMP test
  LEA r0, testjmp
  PUTS

  LEA r1, jmptestok
  JMP r1
  LEA r0, errorstring
  PUTS
  HALT

jmptestok
  LEA r0, okstring
  PUTS
  
;; JSRR test
  LEA r0, testjsrr
  PUTS

  LEA r1, fntestr2
  JSRR r1
  ADD r0, r2, -10
  BRz jsrrtestok
  LEA r0, errorstring
  PUTS
  HALT

jsrrtestok
  LEA r0, okstring
  PUTS

;; JSR test
  LEA r0, testjsr
  PUTS

  JSR fntestr2
  ADD r0, r2, -10
  BRz jsrtestok
  LEA r0, errorstring
  PUTS
  HALT

jsrtestok
  LEA r0, okstring
  PUTS

;; AND test
  LEA r0, testand
  PUTS

  AND r0, r0, 0
  BRz andtestok
  LEA r0, errorstring
  PUTS
  HALT

andtestok
  LEA r0, okstring
  PUTS

;; LDR test
  LEA r0, testldr
  PUTS

  LEA r1, array1
  LDR r2, r1, 1
  ADD r2, r2, -6
  BRz ldrtestok
  LEA r0, errorstring
  PUTS
  HALT

ldrtestok
  LEA r0, okstring
  PUTS

;; STR test
  LEA r0, teststr
  PUTS

  ; get the head of the array and store the value 2 on array[1]
  LEA r0, array1
  AND r1, r1, 0
  ADD r1, r1, 2
  STR r1, r0, 1
  ; check if it stored the value 2 in array[1]
  LDR r2, r0, 1
  ADD r2, r2, -2
  BRz strtestok
  LEA r0, errorstring
  PUTS
  HALT

strtestok
  LEA r0, okstring
  PUTS

;; NOT test
  LEA r0, testnot
  PUTS

  AND r0, r0, 0
  ADD r0, r0, 1
  NOT r0, r0
  BRp nottestok
  LEA r0, errorstring
  PUTS
  HALT

nottestok
  LEA r0, okstring
  PUTS

;; LDI test
  LEA r0, testldisti
  PUTS

  ; load the address of the var2 into the pointer
  LEA r0, var2
  ST  r0, pointer1

  ; get the value at the address stored in r0 and check the value
  LDI r0, pointer1
  ADD r1, r0, -5
  BRz lditestok
  LEA r0, errorstring
  PUTS
  HALT

lditestok
  ; store value on the pointer (pointer1 -> var2 = r0)
  AND r0, r0, 0
  ADD r0, r0, 6
  STI r0, pointer1

  ; check if the value of var2 is 6
  LD r1, var2
  ADD r0, r1, -6
  BRz stitestok
  LEA r0, errorstring
  PUTS
  HALT

stitestok
  LEA r0, okstring
  PUTS
  


  HALT                  ; halt the system

;; ---------- Helper functions

; stores decimal 10 on r2 and return
fntestr2
  AND r2, r2, 0
  ADD r2, r2, 10
  RET

;; ---------- Variables
var1          .blkw 1
var2          .fill 5

pointer1      .blkw 1

array1        .fill 10
              .fill 6
              .fill 30

;; ---------- Constantes

okstring      .stringz "OK!\n"
errorstring   .stringz "ERROR!\n"

testbranchz   .stringz "BRz test... "
testbranchn   .stringz "BRn test... "
testbranchp   .stringz "BRp test... "
testand       .stringz "AND test... "
testlea       .stringz "LEA/PUTS test... "
testadd       .stringz "ADD test... "
testst        .stringz "ST/LD test... "
testjmp       .stringz "JMP test... "
testjsrr      .stringz "JSRR/RET test... "
testjsr       .stringz "JSR test... "
testldr       .stringz "LDR test... "
teststr       .stringz "STR test... "
testnot       .stringz "NOT test... "
testldisti    .stringz "LDI/STI test... "

  .END
