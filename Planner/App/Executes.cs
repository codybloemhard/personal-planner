using System;
using System.Collections.Generic;

namespace Planner
{
    public static class Executes
    {
        public static void Now(string[] com)
        {
            if (com[0] != "now") return;
            Conzole.PrintLine(Schedule.StrDateTime(DateTime.Now));
        }

        public static void Today(string[] com)
        {
            if (com[0] != "date") return;
            Conzole.PrintLine(Schedule.StrDate(DateTime.Now));
        }

        public static void Time(string[] com)
        {
            if (com[0] != "time") return;
            Conzole.PrintLine(Schedule.StrTime(DateTime.Now));
        }

        public static void ShowDeadlines(string[] com)
        {
            if (com.Length < 2) return;
            if (com[0] != "show") return;
            if (com[1] != "deadlines") return;
            if(Schedule.AmountDeadlines() == 0)
            {
                Conzole.PrintLine("No deadlines for you to work on!", ConsoleColor.Magenta);
                return;
            }
            Conzole.PrintLine("Deadlines: ", ConsoleColor.Magenta);
            List<Deadline> dls = new List<Deadline>();
            for (int i = 0; i < Schedule.AmountDeadlines(); i++)
                dls.Add(Schedule.GetDeadline(i));
            dls.Sort((p, q) => p.SecondsUntil().CompareTo(q.SecondsUntil()));
            for (int i = 0; i < dls.Count; i++)
            {
                Deadline l = dls[i];
                Conzole.Print(Schedule.StrDateTime(l.deadline) + " - ", ConsoleColor.Yellow);
                Conzole.Print(l.title + " - ", ConsoleColor.Green);
                Conzole.Print(l.category + "\n", ConsoleColor.Green);
            }         
        }

        public static void AddDeadline(string[] com)
        {
            if (com.Length < 6) return;
            if (com[0] != "add") return;
            if (com[1] != "deadline") return;
            DateTime dt;
            bool ok = Schedule.DateTimeFromString(com[2] + "-" + com[3], out dt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return;
            }
            Deadline dl = new Deadline();
            dl.deadline = dt;
            dl.title = com[4];
            dl.category = com[5];
            Schedule.AddDeadline(dl);
            Schedule.WriteDeadlines();
            Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
        }

        public static void DeleteDeadline(string[] com)
        {
            if (com.Length < 4) return;
            if (com[0] != "delete") return;
            if (com[1] != "deadline") return;
            DateTime dt;
            bool ok = Schedule.DateTimeFromString(com[2] + "-" + com[3], out dt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return;
            }
            for (int i = 0; i < Schedule.AmountDeadlines(); i++)
            {
                Deadline dl = Schedule.GetDeadline(i);
                if(dl.deadline == dt)
                {
                    Schedule.DeleteDeadline(dl);
                    Schedule.WriteDeadlines();
                    Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
                    return;
                }
            }
            Conzole.PrintLine("Could not find deadline!", ConsoleColor.Red);
        }

        public static void ShowDay(string[] com)
        {
            if (com.Length < 2) return;
            if (com[0] != "show") return;
            if (com[1] != "day") return;
            int w = 120, h = 50;
            char[,] view = new char[w, h];
            for (int i = 0; i < w; i++)
                view[i, 0] = '#';
            for (int i = 0; i < w; i++)
                view[i, h - 1] = '#';
            for (int i = 1; i < h; i++)
                view[0, i] = '#';
            for (int i = 1; i < h; i++)
                view[w - 1, i] = '#';

            Conzole.PrintGrid(view);
        }
    }
}