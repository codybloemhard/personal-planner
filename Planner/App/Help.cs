using System;

namespace Planner
{
    public class Help
    {
        public static bool HelpMe(string[] com)
        {
            if (com.Length < 1) return false;
            if (com[0] != "help") return false;
            if(com.Length == 1)
            {
                Conzole.PrintLine("Help: ", ConsoleColor.Magenta);
                Conzole.PrintLine("Type in \"help [command]\" to find help about a command.");
                Conzole.PrintLine("Type in \"help syntax\" to find help about the syntax in help.");
                Conzole.PrintLine("Commands: ");
                Conzole.Enlist(commands);
                return true;
            }
            if(com[1] == "syntax")
            {
                Conzole.PrintLine("Syntax help: ", ConsoleColor.Magenta);
                Conzole.PrintLine(syntaxmsg);
                return true;
            }
            int index = 0;
            bool found = false;
            string command = com[1];
            if (command.Length >= 3)
                for (int i = 2; i < com.Length; i++)
                    command += " " + com[i];
            for(int i = 0; i < commands.Length; i++)
            {
                if(commands[i] == command)
                {
                    index = i;
                    found = true;
                    break;
                }
            }
            if(found && index < syntax.Length)
            {
                Conzole.PrintLine("Syntax: ", ConsoleColor.Magenta);
                Conzole.PrintLine(syntax[index]);
            }
            return true;
        }

        private static string syntaxmsg =
@"Any word found in the syntax, ex ""edit"" or ""deadline"" is a constant you literally type in.
[x, y, z, ...] means that you can choose one of those constants listed.
<Q:name> is a variable, where Q is the type and name is the name of the variable.
The types are: s(string), n(number), t(time), d(date).
A string is just a bunch of letters, no spaces allowed.
A number must be larger or equal to zero.
A time or date is written as ?:?:? where ? are numbers. You can split the ? by "";"" and "":"" and ""-"" and ""/"".
For time and date, the numbers must be correct, ex. time 74:3:5 is not correct, seconds go from 0 to 59.";

        private static string[] commands = new string[] 
        {
            "now", "date", "time",
            "list deadlines", "add deadline", "delete deadline",
            "edit deadline", "list cards", "add card", "delete card",
            "edit card", "inspect card", "clean"
        };

        private static string[] syntax = new string[]
        {
            "now",
            "date",
            "time",
            "list deadlines ([archive, past])",
            "add deadline <t:time> <d:date> <s:title> <s:category>",
            "delete deadline [<t:time>, null] <d:date>",
            "edit deadline [<t:time>, null] <d:date> [title, category] <s:value>\nedit deadline [<t:time>, null] <d:date> deadline <t:time> <d:date> ",
            "list cards ([archive, past]) (<n:amount>)",
            "add card <t:start> <d:start> <t:end> <d:end> <s:title> <s:category>",
            "delete card [<t:time>, null] <d:date>",
            "edit card [<t:time>, null] <d:date> [title, category, content] <s:string>\nedit card [<t:time>, null] <d:date> [start, end] <t:time> <d:date>",
            "inspect card [<t:time>, null], <d:date>",
            "clean [deadlines, cards]"
        };
    }
}