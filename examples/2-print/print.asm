.ORIG x3000

        ;print a character
        LEA r0, mychar
        PUTS

        GETC
        OUT
        HALT

mychar  .stringz "Hello Jacqueline!"
        .end
