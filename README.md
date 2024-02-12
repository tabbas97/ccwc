# Coding Challenge 1 - wc tool from Coding Challenges

## The Challenge - Building wc

The functional requirements for wc are concisely described by it’s man page - give it a go in your local terminal now:

man wc

### Step One
In this step the goal is to write a simple version of wc, that takes the command line option -c and outputs the number of bytes in a file.

>ccwc -c test.txt
  342190 test.txt

### Step Two
In this step the goal is to support the command line option -l that outputs the number of lines in a file.

>ccwc -l test.txt
    7145 test.txt

### Step Three
In this step the goal is to support the command line option -w that outputs the number of words in a file. 

>ccwc -w test.txt
   58164 test.txt

### Step Four
In this step the goal is to support the command line option -m that outputs the number of characters in a file. If the current locale does not support multibyte characters this will match the -c option.

We use wc itself and compare the output to our solution:

>wc -m test.txt
  339292 test.txt

>ccwc -m test.txt
  339292 test.txt

### Step Five
In this step the goal is to support the default option - i.e. no options are provided, which is the equivalent to the -c, -l and -w options.

>ccwc test.txt
    7145   58164  342190 test.txt

### The Final Step
In this step the goal is to support being able to read from standard input if no filename is specified.

>cat test.txt | ccwc -l
    7145

This is achieved by checking the stdin for available data. If anyone has suggestions on handling it better, I'm ears open.
