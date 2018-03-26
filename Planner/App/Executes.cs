using System;
using System.Collections.Generic;

namespace Planner
{
    public static class Executes
    {
        public static bool Now(string[] com)
        {
            if (com[0] != "now") return false;
            Conzole.PrintLine(Schedule.StrDateTime(DateTime.Now));
            return true;
        }

        public static bool Today(string[] com)
        {
            if (com[0] != "date") return false;
            Conzole.PrintLine(Schedule.StrDate(DateTime.Now));
            return true;
        }

        public static bool Time(string[] com)
        {
            if (com[0] != "time") return false;
            Conzole.PrintLine(Schedule.StrTime(DateTime.Now));
            return true;
        }

        public static bool ShowDeadlines(string[] com)
        {
            if (com.Length < 2) return false;
            if (com[0] != "show") return false;
            if (com[1] != "deadlines") return false;
            if(Schedule.AmountDeadlines() == 0)
            {
                Conzole.PrintLine("No deadlines for you to work on!", ConsoleColor.Magenta);
                return false;
            }
            Conzole.PrintLine("Deadlines: ", ConsoleColor.Magenta);
            List<Deadline> dls = new List<Deadline>();
            for (int i = 0; i < Schedule.AmountDeadlines(); i++)
                dls.Add(Schedule.GetDeadline(i));
            dls.Sort((p, q) => p.SecondsLeft().CompareTo(q.SecondsLeft()));
            for (int i = 0; i < dls.Count; i++)
            {
                Deadline l = dls[i];
                Conzole.Print(Schedule.StrDateTime(l.deadline) + " - ", ConsoleColor.Yellow);
                string msg = "";
                int left = l.SecondsLeft();
                int abs = Math.Abs(left);
                int min = 60;
                int hour = min * 60;
                int day = hour * 24;
                if (abs < min * 5)
                    msg = Conzole.PadBefore(abs + "", 4) + " seconds - ";
                else if (abs < hour)
                    msg = Conzole.PadBefore("" + (abs / min), 4) + " minutes - ";
                else if (abs < day * 2)
                    msg = Conzole.PadBefore("" + (abs / hour), 4) + " hours   - ";
                else
                    msg = Conzole.PadBefore("" + (abs / day), 4) + " days    - ";
                if (left > 0) msg = "Left: " + msg;
                else msg = "Past: " + msg;
                ConsoleColor colour;
                if (left > 0) colour = ConsoleColor.White;
                else colour = ConsoleColor.Red;
                Conzole.Print(msg, colour);
                Conzole.Print(Conzole.PadAfter(l.title, 50) + " - ", ConsoleColor.Green);
                Conzole.Print(Conzole.PadAfter(l.category, 20) + "\n", ConsoleColor.Green);
            }
            return true;
        }

        public static bool AddDeadline(string[] com)
        {
            if (com.Length < 6) return false;
            if (com[0] != "add") return false;
            if (com[1] != "deadline") return false;
            DateTime dt;
            bool ok = Schedule.DateTimeFromString(com[2] + "-" + com[3], out dt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            Deadline dl = new Deadline();
            dl.deadline = dt;
            dl.title = com[4];
            dl.category = com[5];
            Schedule.AddDeadline(dl);
            Schedule.WriteDeadlines();
            Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
            return true;
        }

        public static bool DeleteDeadline(string[] com)
        {
            if (com.Length < 4) return false;
            if (com[0] != "delete") return false;
            if (com[1] != "deadline") return false;
            DateTime origDt;
            Deadline deadline;
            int deadlineIndex;
            bool found = false;
            string firstPart;
            if (com[2] == "null")
                firstPart = "0:0:0";
            else firstPart = com[2];
            bool ok = Schedule.DateTimeFromString(firstPart + "-" + com[3], out origDt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            found = Schedule.GetDeadline(origDt, com[2] == "null", out deadline, out deadlineIndex);
            if (found)
            {
                Schedule.DeleteDeadline(deadline);
                Schedule.WriteDeadlines();
                Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
                return true;
            }
            Conzole.PrintLine("Could not find deadline!", ConsoleColor.Red);
            return false;
        }
        //edit deadline oTime oDate atribute nVal/nTime (nDate)
        public static bool EditDeadline(string[] com)
        {
            if (com.Length < 6) return false;
            if (com[0] != "edit") return false;
            if (com[1] != "deadline") return false;
            DateTime origDt;
            Deadline deadline;
            int deadlineIndex;
            bool found = false;
            string firstPart;
            if (com[2] == "null")
                firstPart = "0:0:0";
            else firstPart = com[2];
            bool ok = Schedule.DateTimeFromString(firstPart + "-" + com[3], out origDt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            found = Schedule.GetDeadline(origDt, com[2] == "null", out deadline, out deadlineIndex);
            if (!found)
            {
                Conzole.PrintLine("Deadline not found!", ConsoleColor.Red);
                return false;
            }
            if (!(com[4] == "deadline" || com[4] == "title"
                || com[4] == "category"))
            {
                Conzole.PrintLine("Atribute not found!", ConsoleColor.Red);
                Conzole.PrintLine("Atributes: deadline, title, category.", ConsoleColor.Red);
                return false;
            }
            if (com[4] == "deadline")
            {
                if (com.Length < 7)
                {
                    Conzole.PrintLine("Not enough arguments! Give a new time and date!", ConsoleColor.Red);
                    return false;
                }
                DateTime dt;
                ok = Schedule.DateTimeFromString(com[5] + "-" + com[6], out dt);
                if (!ok)
                {
                    Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                    return false;
                }
                deadline.deadline = dt;
            }
            else if (com[4] == "title")
                deadline.title = com[5];
            else if (com[4] == "category")
                deadline.category = com[5];
            Schedule.EditDeadline(deadlineIndex, deadline);
            Schedule.WriteDeadlines();
            Conzole.PrintLine("Succes", ConsoleColor.Magenta);
            return true;
        }
        
        public static bool ShowCards(string[] com)
        {
            if (com.Length < 3) return false;
            if (com[0] != "show") return false;
            if (com[1] != "cards") return false;
            uint count;
            bool ok = uint.TryParse(com[2], out count);
            if (!ok)
            {
                Conzole.PrintLine("Could not convert \"" + com[2] + "\" to a uint.");
                return false;
            }
            int max = Schedule.AmountCards();
            if (count == 0) count = (uint)max;
            if (count > max) count = (uint)max;
            Conzole.PrintLine("Found " + count + " cards.", ConsoleColor.Magenta);
            List<Card> cards = new List<Card>();
            for (int i = 0; i < Schedule.AmountCards(); i++)
                cards.Add(Schedule.GetCard(i));
            cards.Sort((p, q) => p.start.CompareTo(q.end));
            for(int i = 0; i < count; i++)
            {
                Card c = cards[i];
                Conzole.Print(Schedule.StrDateTime(c.start) + " [...] ", ConsoleColor.Yellow);
                Conzole.Print(Schedule.StrDateTime(c.end) + " - ", ConsoleColor.Yellow);
                Conzole.Print(Conzole.PadAfter(c.title, 30) + " - ");
                Conzole.Print(Conzole.PadAfter(c.category, 20));
                Conzole.Print("\n");
            }
            return true;
        }
        //add card startTime startDate endTime endDate title category
        public static bool AddCard(string[] com)
        {
            if (com.Length < 8) return false;
            if (com[0] != "add") return false;
            if (com[1] != "card") return false;
            DateTime startDt, endDt;
            string firstPart;
            if (com[2] == "null")
                firstPart = "0:0:0";
            else firstPart = com[2];
            bool ok = Schedule.DateTimeFromString(firstPart + "-" + com[3], out startDt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            if (com[4] == "null")
                firstPart = "0:0:0";
            else firstPart = com[4];
            ok = Schedule.DateTimeFromString(firstPart + "-" + com[5], out endDt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            Card card = new Card();
            card.start = startDt;
            card.end = endDt;
            card.title = com[6];
            card.content = "";
            card.category = com[7];
            Schedule.AddCard(card);
            Schedule.WriteCards();
            Conzole.PrintLine("Succes", ConsoleColor.Magenta);
            return true;
        }

        public static bool ShowDay(string[] com)
        {
            if (com.Length < 2) return false;
            if (com[0] != "show") return false;
            if (com[1] != "day") return false;
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
            return true;
        }
    }
}