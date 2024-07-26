5 REM Hello, World! in BASIC
6 X = 1
7 Y = X + 2
8 PRINT "X = "; X; "Y = "; Y
9 GOSUB 65
10 N=10
15 INPUT "What is your name? "; NAME$
16 IF NAME$ = "STOP" THEN END ELSE X = 99
20 FOR I=1 TO N STEP 2
30 PRINT "Hello, "; NAME$
40 NEXT I
45 GOTO 60
49 X = 5
50 X = (X + 1) * (X  + 1)
60 Y = X + 1
65 PRINT "X = "; X; "Y = "; Y: RETURN
70 GOTO 50
