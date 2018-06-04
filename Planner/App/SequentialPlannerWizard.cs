using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Planner
{
    public static class SequentialPlannerWizard
    {
        public static bool Run(string[] com)
        {
            if (com[0] != "run") return false;
            if (com[1] != "sequentialplanner") return false;
            Conzole.PrintLine("Welcome to the Sequential Planner Wizard.", ConsoleColor.Magenta);
            Conzole.PrintLine("Type \"exit\" to quit this wizard.", ConsoleColor.Magenta);
            string s;
            DateTime dt;
            bool ok = false;
            while (true)
            {
                Conzole.PrintLine("Enter the first day: ", ConsoleColor.Magenta);
                Conzole.Print("[SequentialPlanner]:: ", ConsoleColor.Cyan);
                s = Console.ReadLine();
                if (s == "exit") return true;
                ok = Schedule.DateFromString(s, out dt);
                if (ok) break;
                Conzole.PrintLine("Could not convert to correct datetime!", ConsoleColor.Red);
            }
            Conzole.PrintLine("You can use the commands: ", ConsoleColor.Magenta);
            Conzole.PrintLine("exit // exit this wizard.", ConsoleColor.Magenta);
            Conzole.PrintLine("add <s:timeslotname> <s:name> <s:category> // add a event on current day at timeslot.", ConsoleColor.Magenta);
            Conzole.PrintLine("next // increment day.", ConsoleColor.Magenta);
            Conzole.PrintLine("skip <n:x> // increment x days.", ConsoleColor.Magenta);
            Conzole.PrintLine("save // save the planned events and exit.");
            while (true)
            {
                Conzole.PrintLine("Current date: " + Schedule.StrDateTime(dt), ConsoleColor.Yellow);
                Conzole.Print("[SequentialPlanner]:: ", ConsoleColor.Cyan);
                s = Console.ReadLine();
                if (s == "exit") return true;
                if (s == "save") break;
                if (s == "next")
                {
                    dt = dt.AddDays(1);
                }
                else
                {
                    string[] lcoms = Interperter.ExtractCommand(s);
                    if (lcoms.Length == 0) continue;
                    if(lcoms[0] == "skip" && lcoms.Length == 2)
                    {
                        int i = 0;
                        int.TryParse(lcoms[1], out i);
                        if (i <= 1) continue;
                        dt = dt.AddDays(i);
                    }
                    else if(lcoms[0] == "add" && lcoms.Length == 4)
                    {
                        TimeSlot ts;
                        ok = Schedule.timeslots.Get(lcoms[1], out ts);
                        if (!ok)
                        {
                            Conzole.PrintLine("Could not find timeslot: " + lcoms[1], ConsoleColor.Red);
                            continue;
                        }
                        DateTime start = ts.StartToDateTime(dt);
                        DateTime end = ts.EndToDateTime(dt);
                        Card card = new Card();
                        card.start = start;
                        card.end = end;
                        card.title = lcoms[2];
                        card.category = lcoms[3];
                        card.content = "";
                        Schedule.cards.Add(card);
                        Conzole.PrintLine("Card added.", ConsoleColor.Magenta);
                    }
                }
            }
            Schedule.cards.Write();
            Conzole.PrintLine("All cards saved!", ConsoleColor.Yellow);
            return true;
        }
    }
}