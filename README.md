# rslc3vm - Implements a simple LC3 virtual machine in Rust.

This software implements a simple LC3 (Little Computer 3) virtual machine in Rust.
https://en.wikipedia.org/wiki/Little_Computer_3

It is inspired and based on the blog entry by Andrei Ciobanu:
https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c

Also, my first baby steps in Rust :-)

## Build/run

On the console execute
    `cargo run data/tputs.lc3obj`

## Sample Programs

- programs are encoded as little-endian

### `tputs.lc3obj`

- loads address of nul-terminated string
- prints string to console
- terminates

```
0x3000: 0xE002  // LEA R0, 0x02 (R0 = 0x3003 = 0x3001(PC) + 0x02)
0x3001: 0xF022  // TRAP TPUTS
0x3002: 0xF025  // TRAP HALT

Data
0x3003: 0x0048  // 'H'
0x3004: 0x0045  // 'E'
0x3005: 0x004C  // 'L'
0x3006: 0x004C  // 'L'
0x3007: 0x004F  // 'O'
0x3008: 0x0000  // NUL
```

The following two sample programs are from Andrei's code.

### `sum.lc3obj`

- reads two `u16` from the console
- adds them together
- writes the result to the console
- terminates

```
0xF026    //  1111 0000 0010 0110  TRAP tinu16      ;read an uint16_t in R0
0x1220    //  0001 0010 0010 0000  ADD R1,R0,x0     ;add contents of R0 to R1
0xF026    //  1111 0000 0010 0110  TRAP tinu16      ;read an uint16_t in R0
0x1240    //  0001 0010 0010 0000  ADD R1,R1,R0     ;add contents of R0 to R1
0x1060    //  0001 0000 0110 0000  ADD R0,R1,x0     ;add contents of R1 to R0
0xF027    //  1111 0000 0010 0111  TRAP toutu16     ;show the contents of R0 to stdout
0xF025    //  1111 0000 0010 0101  HALT             ;halt
```

### `simple_program.lc3obj`

- calculates the sum of the ten values in R1
- terminates

```
0x5260    //  0101 0010 0110 0000             AND R1,R1,x0    ;clear R1, to be used for the running sum
0x5920    //  0101 1001 0010 0000             AND R4,R4,x0    ;clear R4, to be used as a counter
0x192A    //  0001 1001 0010 1010             ADD R4,R4,xB    ;load R4 with #10, the number of times to add
0xE406    //  1110 0100 0000 0110             LEA R2,x7       ;load the starting address of the data
0x6680    //  0110 0110 1000 0000     LOOP    LDR R3,R2,x0    ;load the next number to be added
0x14A1    //  0001 0100 1010 0001             ADD R2,R2,x1    ;increment the pointer
0x1243    //  0001 0010 0100 0011             ADD R1,R1,R3    ;add the next number to the running sum
0x193F    //  0001 1001 0011 1111             ADD R4,R4,x-1   ;decrement the counter
0x03FB    //  0000 0011 1111 1011             BRp LOOP        ;do it again if the counter is not yet zero
0xF025    //  1111 0000 0010 0101             HALT            ;halt

Data at 0x300A
0x0001 /* 1 */
0x0002 /* +2 = 3 */
0x0001 /* +1 = 4 */
0x0002 /* +2 = 6 */
0x0003 /* +3 = 9 */
0x0001 /* +1 = 10 */
0x0002 /* +2 = 12 */
0x0001 /* +1 = 13 */
0x0002 /* +2 = 15 */
0x0001 /* +1 = 16 */
```
