.TH man 8 "08 May 2019" "0.2" "pplanner man page"
.SH NAME
pplanner - Personal Planner
.SH SYNOPSIS
.B
pplanner 
.RB [ \--help ] 
.RB [ \-e 
.IR command
.RB [ \-i 
.IR inputs
]]
.SH DESCRIPTION
pplanner is a TUI/CLi tool for managing your time.
You can use it by running pplanner, or by using it CLI style with flags and arguments.
Running it and using the TUI is recommended, CLI is recommended for usage in scripts.
.SH OPTIONS
.TP
.B \--help
Gives a help text.
.TP
.B \-e
Execute pplanner command. It can ask for input and will output the results on stdout.
.TP
.B \-i
Only valid when
.B \-e
is used. You provide the inputs for all the prompts and it will fill it in for you.
Inputs are separated by commas (,).
.SH CLI EXAMPLES
.PP
pplanner
.PP
pplanner -e status
.PP
pplanner -e 'clean' -i 'y'
.PP
pplanner -e 'mk todo'
.PP
pplanner -e 'mk todo' -i 'Need to do task,16,todo'
.PP
pplanner -e 'mk todo' -i 'Less important,32,long'
.SH SEE ALSO
.PP
pplanner --help
.PP
pplanner -e help
.PP
pplanner -e 'ls commands'
.PP
pplanner -e 'help(now)'
.PP
pplanner -e 'help(ls points)'
.PP
ect
.SH BUGS
.SH AUTHOR
Cody Bloemhard (cblmhrd@live.nl)
https://github.com/ocdy1001/PersonalPlanner
