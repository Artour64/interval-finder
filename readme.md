# Interval Finder
A CLI tool for finding musical just intonation interval ratios.  
To do more analysis on an interval you found, you may use the [interval-info](https://github.com/Artour64/interval-info) tool.  

## Arguments
Running without arguments displays a message that tells you to input an interval.  
Invalid arguments are ignored and if the second part of the argument can not be parsed, the default will be used in most cases.  
The last argument is the target interval which is the interval to search around.  
By default this argument is in cents (integer or decimal) but the flags -f, -d, -e, and their long forms change that as documented below*.  

*Note: for those options, the flag does not have to be beside the last argument like in the example.  
As long as the last argument is in the correct form and the corresponding flag is present somewhere in the arguments, it will work.  
For example, calling `interval-finder -n -f -l 19 3/2` is a valid way of searching for 19-limit intervals that are close to (and including) the just fifth (3/2) and not displaying the summary.  
In this case, `-f` is the fraction flag and `3/2` is the target interval as a fraction.  

### Flags
-h --help  
 Display this help message.  

-n --no-summary  
 Do not display summary at the end.  
 
 -f --fraction \[positive integer\]/\[positive integer\]  
 Alternative way of inputting the target interval to search for.  
 Instead of cents, input the interval as a fraction.  
 e.g. `-f 3/2` or `--fraction 3/2`  
 
 -d --decimal \[decimal number\]  
 Alternative way of inputting the target interval to search for.  
 Instead of cents, input the interval as a decimal number form of the fraction.  
 e.g. `-d 1.5` or `--decimal 1.5`  
 
 -e --edo \[positive integer\]/\[positive integer\]  
 Alternative way of inputting the target interval to search for.  
 Instead of cents, input the interval as a note of an EDO scale.  
 e.g. `-e 7/12` or `--edo 7/12`  

### options with arguments
 -l --limit \[positive integer\]  
 A filter for the highest tuning limit, which is the highest prime factor allowed in either numerator or denominator.  
 Filter is disabled by default allowing any tuning.  
 e.g. `-l 5` or `--limit 5`  

 -r --search-cent-radius \[decimal number\]  
 The radius around the target interval that will be searched in cents.  
 Negative numbers will be made positive.  
 The default is 40.  
 e.g. `-r 5` or `--search-cent-radius 5`  

-m --max-iter \[positive integer\]  
 Max numerator and denominators to check.  
 Default is 30.  
 e.g. `-m 100` or `--max-iter 100`  
