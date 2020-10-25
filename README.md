# PROJECT IS NO LONGER IN DEVELOPMENT

# rustic
BASIC Interpreter written in Rust

This project intends to be a rewrite of the ArduinoBASIC project using the freedom granted by Rust.
It aims to be a more professional and usable form of the basic interpreter originally presented in that project.

Current Status: Non-functional. Only code currently executed is LET with single integer statements.

# Installation 
```
git clone https://github.com/666666t/rustic.git
```

# Use
In its current form, the program intends to read and tokenize statements from a file, as opposed to manual entry.

Usage: `cargo run <path>`

The only format required for proper reading is a line-delimited file containing commands in the format  
`<line number> <command name> <parameter>`  

Example:

```
10 LET A = 5  
20 PRINT "Given Text"  
30 GOTO 20  
```

#Commands:

LET (Syntax: `LET <name> = <integer variable>`): Variable assignments, and the target of improvement over the original project.  
Intended Improvements:  
  -Proper Expression Handling:  
    -Old version: only supported LET statements were direct assignments (LET A = 5) or single additions (LET A = B + 0)  
    -New version aim: Proper parsing of variable names and expression handling at runtime (LET A = B * 8 - C)  
PRINT (Syntax: `PRINT <variable name> or PRINT "STRING"`) Prints given expressions to the screen.  
Potential Updates:  
  -Optional coordinate system, potential split to a LOCATE command  
    -Would allow recurrent printing of a variable to a single location rather than bounds of terminal scrolling  
GOTO (Syntax: `GOTO <line number>`): Provides jump to given point in program by line number  

This is the initial command set, if/when those are implemented (IF/ELSE)/(FOR/TO/STEP)/WHILE will be planned.  
