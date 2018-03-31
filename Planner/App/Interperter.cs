using System;
using System.Collections.Generic;
using System.IO;

namespace Planner
{
    public class Interperter
    {
        private List<string> strings;
        private List<Func<string[], bool>> executes;

        public Interperter()
        {
            strings = new List<string>();
            executes = new List<Func<string[], bool>>();
            executes.Add(Help.HelpMe);
            executes.Add(Executes.Now);
            executes.Add(Executes.Date);
            executes.Add(Executes.Time);
            executes.Add(Executes.ShowDay);
            executes.Add(Executes.ListDeadlines);
            executes.Add(Executes.AddDeadline);
            executes.Add(Executes.DeleteDeadline);
            executes.Add(Executes.EditDeadline);
            executes.Add(Executes.ListCards);
            executes.Add(Executes.AddCard);
            executes.Add(Executes.DeleteCard);
            executes.Add(Executes.EditCard);
            executes.Add(Executes.InspectCard);
            //run the app
            Conzole.SetDimensions(120, 2000);
            Conzole.SetColour(ConsoleColor.Green);
            Introduce();
            AskCommand();
        }
        
        private void Introduce()
        {
            Conzole.PrintLine("Personal Planner.", ConsoleColor.Cyan);
            Conzole.PrintLine("Made by Cody Bloemhard.", ConsoleColor.Cyan);
            Conzole.PrintLine("type \"help\" and press enter for help.", ConsoleColor.Cyan);
        }
        
        private void AskCommand()
        {
            Conzole.Print("[CONSOLE]:: ", ConsoleColor.Cyan);
            byte[] inputBuffer = new byte[2048];
            Stream inputStream = Console.OpenStandardInput(inputBuffer.Length);
            Console.SetIn(new StreamReader(inputStream, Console.InputEncoding, false, inputBuffer.Length));
            string raw = Console.ReadLine();
            raw = raw.ToLower();
            if (raw == "exit") return;
            if(raw == "clear")
            {
                Console.Clear();
                Introduce();
                AskCommand();
                return;
            }
            if (raw == "")
            {
                AskCommand();
                return;
            }
            string[] command = ExtractCommand(raw);
            for (int i = 0; i < executes.Count; i++)
                if (executes[i](command)) break;
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