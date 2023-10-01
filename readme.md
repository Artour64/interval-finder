#Interval Finder

A CLI tool for finding musical just intonation interval ratios.


##Arguments

Running without arguments displays the help message.

Invalid arguments are ignored and if the second part of the argument can not be parsed, the default will be used in most cases.

-h --help
 Display this help message.

-n --no-summary
 Do not display summary at the end.

-m --max-iter [positve integer]
 Max numerator and denominators to check.
 Default is 30
 e.g.:
 -m 100
 or
 --max-iter 100

-l --limit [positive integer]
 A filter for the highest tuning limit, which is the highest prime factor allowed in either numerator or denominator.
 Filter is disabled by default allowing any tuning.
 e.g.:
 -l 5
 or
 --limit 5

 -t --target [decimal number]
 The target interval to search for and around in cents.
 A cent is a hundredth of a 12tet (common tuning) semitone.
 100 cents is a 12tet (common tuning) semitone, 1200 is an pure octave, 0 is a unison (same note).
 Negative numbers represent downward/flipped intervals.
 Allows for decimals like 386.314 for more precision.
 e.g.:
 -t 700
 or
 --target 700

 -r --search-cent-radius [decimal number]
 The radius around the target interval that will be searched in cents.
 Negative numbers will be made positive.
 The default is 40.
 e.g.:
 -r 5
 or
 --search-cent-radius 5

 -d --decimal [decimal number]
 Alternative way of inputting the target interval to search for.
 Instead of cents, input the interval as a decimal number form of the fraction.
 e.g.:
 -d 1.5
 or
 --decimal 1.5

 -f --fraction [positive integer]/[positive integer]
 Alternative way of inputting the target interval to search for.
 Instead of cents, input the interval as a fraction.
 -f 3/2
 or
 --fraction 3/2
