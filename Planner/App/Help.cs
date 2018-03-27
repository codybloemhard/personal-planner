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
                Conzole.PrintLine("Commands: ");
                string[] coms = new string[] { "now", "date", "time",
                "show deadlines", "add deadline", "delete deadline",
                "edit deadline", "show cards", "add card", "delete card",
                "edit card", "inspect card"};
                Conzole.Enlist(coms);
            }

            return true;
        }
    }
}