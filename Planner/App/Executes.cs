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
            Schedule.deadlines.Write();
            return true;
        }

        public static bool Date(string[] com)
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

        public static bool ListDeadlines(string[] com)
        {
            if (com.Length < 2) return false;
            if (com[0] != "list") return false;
            if (com[1] != "deadlines") return false;
            DeadlineFile df = Schedule.deadlines;
            DateTime limit = DateTime.MaxValue;
            if (com.Length >= 3 && com[2] == "archive")
                df = Schedule.deadlinesArchive;
            else if (com.Length >= 3 && com[2] == "past")
                limit = DateTime.Now.AddSeconds(-1);
            else if (com.Length >= 3)
                limit = Logic.Limit(com[2]);
            if (df.Size() == 0)
            {
                Conzole.PrintLine("No deadlines to show!", ConsoleColor.Magenta);
                return false;
            }
            Conzole.PrintLine("Deadlines: ", ConsoleColor.Magenta);
            List<Deadline> dls = new List<Deadline>();
            for (int i = 0; i < df.Size(); i++)
            {
                Deadline d = df.Get(i);
                if(d.deadline <= limit)
                    dls.Add(d);
            }
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
            Schedule.deadlines.Add(dl);
            Schedule.deadlines.Write();
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
            found = Schedule.deadlines.Get(origDt, com[2] == "null", out deadline, out deadlineIndex);
            if (!found)
            {
                Conzole.PrintLine("Could not find deadline!", ConsoleColor.Red);
                return false;
            }
            Conzole.PrintLine("Deleting deadline " + deadline.title + ".", ConsoleColor.Magenta);
            bool sure = Conzole.AreYouSure();
            if (!sure)
            {
                Conzole.PrintLine("Did not delete anything.", ConsoleColor.Magenta);
                return false;
            }
            Schedule.deadlinesArchive.Add(deadline);
            Schedule.deadlinesArchive.Write();
            Schedule.deadlines.Delete(deadline);
            Schedule.deadlines.Write();
            Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
            return true;
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
            found = Schedule.deadlines.Get(origDt, com[2] == "null", out deadline, out deadlineIndex);
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
            Schedule.deadlines.Edit(deadlineIndex, deadline);
            Schedule.deadlines.Write();
            Conzole.PrintLine("Succes", ConsoleColor.Magenta);
            return true;
        }
        
        public static bool ListCards(string[] com)
        {
            if (com.Length < 2) return false;
            if (com[0] != "list") return false;
            if (com[1] != "cards") return false;
            CardFile cf = Schedule.cards;
            DateTime limit = DateTime.MaxValue;
            int argIndex = 2;
            if(com.Length >= 3)
            {
                if (com[2] == "archive")
                {
                    cf = Schedule.cardsArchive;
                    argIndex = 3;
                }
                else if(com[2] == "past")
                {
                    limit = DateTime.Now.AddSeconds(-1);
                    argIndex = 3;
                }
                else limit = Logic.Limit(com[2]);
            }
            uint count;
            if (com.Length == argIndex + 1 && limit == DateTime.MaxValue)
            {
                bool ok = uint.TryParse(com[argIndex], out count);
                if (!ok)
                {
                    Conzole.PrintLine("Could not convert \"" + com[argIndex] + "\" to a uint.", ConsoleColor.Red);
                    return false;
                }
            }
            else count = 0;
            List<Card> cards = new List<Card>();
            for (int i = 0; i < cf.Size(); i++)
            {
                Card c = cf.Get(i);
                if (c.start <= limit) cards.Add(c);
            }
            cards.Sort((p, q) => p.start.CompareTo(q.end));
            int max = cards.Count;
            if (count == 0) count = (uint)max;
            if (count > max) count = (uint)max;
            Conzole.PrintLine("Found " + count + " cards.", ConsoleColor.Magenta);   
            for(int i = 0; i < count; i++)
            {
                Card c = cards[i];
                Conzole.Print(Schedule.StrDateTime(c.start) + " >> ", ConsoleColor.Yellow);
                Conzole.Print(Schedule.StrDateTime(c.end) + " - ", ConsoleColor.Yellow);
                string msg;
                bool notPast = Logic.GetDayMessage(c.start, out msg);
                ConsoleColor col = notPast ? ConsoleColor.White : ConsoleColor.Red;
                Conzole.Print(msg + " - ", col);
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
            Schedule.cards.Add(card);
            Schedule.cards.Write();
            Conzole.PrintLine("Succes", ConsoleColor.Magenta);
            return true;
        }

        public static bool DeleteCard(string[] com)
        {
            if (com.Length < 4) return false;
            if (com[0] != "delete") return false;
            if (com[1] != "card") return false;
            DateTime origDt;
            Card card;
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
            found = Schedule.cards.Get(origDt, com[2] == "null", out card, out deadlineIndex);
            if (!found)
            {
                Conzole.PrintLine("Could not find card!", ConsoleColor.Red);
                return false;
            }
            Conzole.PrintLine("Deleting card " + card.title + ".", ConsoleColor.Magenta);
            bool sure = Conzole.AreYouSure();
            if (!sure)
            {
                Conzole.PrintLine("Did not delete anything.", ConsoleColor.Magenta);
                return false;
            }
            Schedule.cardsArchive.Add(card);
            Schedule.cardsArchive.Write();
            Schedule.cards.Delete(card);
            Schedule.cards.Write();
            Conzole.PrintLine("Succes!", ConsoleColor.Magenta);
            return true;
        }
        
        public static bool EditCard(string[] com)
        {
            if (com.Length < 6) return false;
            if (com[0] != "edit") return false;
            if (com[1] != "card") return false;
            DateTime origDt;
            Card card;
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
            found = Schedule.cards.Get(origDt, com[2] == "null", out card, out deadlineIndex);
            if (!found)
            {
                Conzole.PrintLine("Card not found!", ConsoleColor.Red);
                return false;
            }
            if (!(com[4] == "start" || com[4] == "end"
                || com[4] == "title" || com[4] == "category"
                || com[4] == "content"))
            {
                Conzole.PrintLine("Atribute not found!", ConsoleColor.Red);
                Conzole.PrintLine("Atributes: start, end, title, category, content.", ConsoleColor.Red);
                return false;
            }
            if (com[4] == "start" || com[4] == "end")
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
                if (com[4] == "start") card.start = dt;
                else card.end = dt;
            }
            else if (com[4] == "title")
                card.title = com[5];
            else if (com[4] == "category")
                card.category = com[5];
            else if (com[4] == "content")
            {
                card.content = "";
                card.content += com[5];
                if(com.Length >= 7)
                    for (int i = 6; i < com.Length; i++)
                        card.content += " " + com[i];
            }
            Schedule.cards.Edit(deadlineIndex, card);
            Schedule.cards.Write();
            Conzole.PrintLine("Succes", ConsoleColor.Magenta);
            return true;
        }

        public static bool InspectCard(string[] com)
        {
            if (com.Length < 3) return false;
            if (com[0] != "inspect") return false;
            if (com[1] != "card") return false;
            DateTime dt;
            string firstPart;
            if (com[2] == "null")
                firstPart = "0:0:0";
            else firstPart = com[2];
            bool ok = Schedule.DateTimeFromString(firstPart + "-" + com[3], out dt);
            if (!ok)
            {
                Conzole.PrintLine("Your date/time is incorrect!", ConsoleColor.Red);
                return false;
            }
            Card card;
            int index = 0;
            bool found = Schedule.cards.Get(dt, com[2] == "null", out card, out index);
            if (!found)
            {
                Conzole.PrintLine("Card not found", ConsoleColor.Red);
                return false;
            }
            Conzole.Print("Title: ", ConsoleColor.Magenta);
            Conzole.PrintLine(Conzole.PadAfter(card.title, 100));
            Conzole.Print("Category: ", ConsoleColor.Magenta);
            Conzole.PrintLine(Conzole.PadAfter(card.category, 97));
            Conzole.Print("Start: ", ConsoleColor.Magenta);
            Conzole.PrintLine(Schedule.StrDateTime(card.start), ConsoleColor.Yellow);
            Conzole.Print("End: ", ConsoleColor.Magenta);
            Conzole.PrintLine(Schedule.StrDateTime(card.end), ConsoleColor.Yellow);
            Conzole.Print("Day: ", ConsoleColor.Magenta);
            Conzole.PrintLine(card.start.DayOfWeek.ToString());
            Conzole.Print("Relativeness: ", ConsoleColor.Magenta);
            string msg;
            bool notPast = Logic.GetDayMessage(card.start, out msg);
            ConsoleColor col = notPast ? ConsoleColor.White : ConsoleColor.Red;
            Conzole.PrintLine(msg, col);
            TimeSpan res = card.end - card.start;
            Conzole.Print("Duration: ", ConsoleColor.Magenta);
            Conzole.PrintLine(res.Hours + " hours, " + res.Minutes + " minutes and " + res.Seconds + " seconds." , ConsoleColor.Yellow);
            Conzole.PrintLine("Content: ", ConsoleColor.Magenta);
            Conzole.PrintLine(card.content);
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