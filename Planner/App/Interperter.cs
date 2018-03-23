using System;
using System.Collections.Generic;

namespace Planner
{
    public class Interperter
    {
        private List<string> strings;
        private List<Action<string[]>> executes;

        public Interperter()
        {
            strings = new List<string>();
            executes = new List<Action<string[]>>();
            executes.Add(Executes.Now);
            executes.Add(Executes.Today);
            executes.Add(Executes.Time);
            executes.Add(Executes.ShowDay);
            executes.Add(Executes.ShowDeadlines);
            executes.Add(Executes.AddDeadline);
            executes.Add(Executes.DeleteDeadline);
            //run the app
            Conzole.SetDimensions(120, 2000);
            Conzole.SetColour(ConsoleColor.Green);
            Conzole.PrintLine("Personal Planner");
            Conzole.PrintLine("Made by Cody Bloemhard");
            AskCommand();
        }

        private void AskCommand()
        {
            Conzole.Print("[CONSOLE]:: ", ConsoleColor.Cyan);
            string raw = Conzole.GetLine();
            raw = raw.ToLower();
            if (raw == "exit") return;
            if (raw == "") AskCommand();
            string[] command = ExtractCommand(raw);
            for (int i = 0; i < executes.Count; i++)
                executes[i](command);
            AskCommand();
        }
        
        private string[] ExtractCommand(string s)
        {
            strings.Clear();
            string temp = "";
            for(int i = 0; i < s.Length; i++)
            {
                if (s[i] == ' ' && temp != "")
                {
                    strings.Add(temp);
                    temp = "";
                }
                else if(s[i] != ' ') temp += s[i];
            }
            if(temp != "") strings.Add(temp);
            return strings.ToArray();
        }

        private void PrintCommand(string[] command)
        {
            for (int i = 0; i < command.Length; i++)
                Conzole.Print("\"" + command[i] + "\" ");
            Conzole.PrintLine("");
        }
    }
}