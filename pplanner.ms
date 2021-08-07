.TH man 8 "26 March 2021" "0.3" "pplanner man page"
.SH NAME
pplanner - Personal Planner
.SH SYNOPSIS
.B
pplanner
.RB [ \--help ]
.RB [ \-i
.IR inputs
]]
.SH DESCRIPTION
pplanner is a CLI/TUI tool for managing your time.
.SH OPTIONS
.TP
.B \--help
Gives a help text.
.TP
.B \-i
You provide the inputs for all the prompts and it will fill it in for you.
Inputs are separated by commas (,).
.SH CLI EXAMPLES
.PP
pplanner status
.PP
pplanner mk todo
.PP
pplanner mk todo -i 'Need to do task,16,todo'
.PP
pplanner mk todo -i 'Less important,32,long'
.SH SEE ALSO
.PP
pplanner --help
.PP
pplanner ls commands
.PP
pplanner 'help(now)'
.PP
pplanner 'help(ls points)'
.PP
ect
.SH BUGS
.SH AUTHOR
Cody Bloemhard (cblmhrd@live.nl)
https://github.com/ocdy1001/PersonalPlanner
