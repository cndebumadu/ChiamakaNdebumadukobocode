
the dot decision

At line 155.I check for a `.` followed by a digit using
`self.peek() == '.' && self.peek_next().is_digit(10)`. For numbers like 5. it will check for the decimal place and peek if their is a number after it if not it will only cosume the 5 and leave the rest. it follows section 1.4, which states that 5. is not a number nd that kobo has no DOT token at all

Line counter changes

Incremented self.line at line 105 for a new line inside scan token function and then againa at line 125 inside the string token for multiline strings. 
For a file ending in two blank lines, my EOF token still carries the line of the last real token, not the file's actual last line. This is
computed at src/scanner.rs:37, where run() takes self.tokens.last().map(|t| t.line).unwrap_or(1) AFTER the scan loop finishes, rather than using whatever self.line happens to be at that point. section 6.1 requires this because a trailing newline left by an editor should not shift line numbers that tests depend on.


A real bug/ my problems

the scanner didn't recognise double characters so from line 70 to line 89. so each of them neaded to look ahead before deciding what the token should be.so i made it check for a '=' operator.
when doing the number peek ahead function i just used peek() instead of peek_next()to see after the decimal point which placed a problem.

I also had looping issues where lines wouldnt function properly because they wher inside a loop 
line 136 - line 140. i moved them ouside the loop or condition it was in.