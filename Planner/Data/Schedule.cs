using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;

namespace Planner
{
    public static class Schedule
    {
        public static DeadlineFile deadlines, deadlinesArchive;
        public static CardFile cards, cardsArchive;
        public static TimeSlotFile timeslots, timeslotsArchive;

        static Schedule()
        {
            deadlines = new DeadlineFile("deadlineData");
            cards = new CardFile("cardData");
            deadlinesArchive = new DeadlineFile("deadlineArchiveData");
            cardsArchive = new CardFile("cardsArchiveData");
            timeslots = new TimeSlotFile("timerangeData");
            timeslotsArchive = new TimeSlotFile("timerangeArchiveData");
        }

        public static void InitSchedule()
        {
            deadlines.Load();
            cards.Load();
        }

        public static bool SameDateTime(DateTime org, bool onlyDate, DateTime cmp)
        {
            if (org == cmp)
                return true;
            if (onlyDate && org.Day == cmp.Day
                && org.Month == cmp.Month
                && org.Year == cmp.Year)
                return true;
            return false;
        }

        public static DateTime ReadDateTime(BinaryReader r)
        {
            int[] datetime = new int[6];
            for (int j = 0; j < 6; j++)
                datetime[j] = r.ReadInt32();
            DateTime dt = new DateTime(datetime[0], datetime[1], datetime[2], 
                datetime[3], datetime[4], datetime[5]);
            return dt;
        }

        public static void WriteDateTime(BinaryWriter w, DateTime t)
        {
            w.Write(t.Year);
            w.Write(t.Month);
            w.Write(t.Day);
            w.Write(t.Hour);
            w.Write(t.Minute);
            w.Write(t.Second);
        }

        public static float MinutesToFloat(DateTime t)
        {
            float minutes = t.Hour * 60 + t.Minute;
            return minutes / (60 * 24);
        }

        public static int DaySinceMonday(DateTime t)
        {
            DateTime monday = FirstDayOfTheWeek(t);
            return (monday - t).Days;
        }

        public static DateTime FirstDayOfTheWeek(DateTime t)
        {
            int diff = (7 + (t.DayOfWeek - DayOfWeek.Monday)) % 7;
            return t.AddDays(-1 * diff).Date;
        }

        public static bool IsInThisWeek(DateTime t, DateTime monday)
        {
            int diff = (t - monday).Days;
            return diff < 7 && diff >= 0;
        }

        public static DateTime NextDay(DateTime t, DayOfWeek day)
        {
            int max = 20;//voor gekkigheden
            while(max >= 0)
            {
                max--;
                t = t.AddDays(1);
                if (t.DayOfWeek == day) return t;
            }
            return DateTime.Now;
        }

        public static DateTime PrevDay(DateTime t, DayOfWeek day)
        {
            int max = 20;//voor gekkigheden
            while (max >= 0)
            {
                max--;
                t = t.AddDays(-1);
                if (t.DayOfWeek == day) return t;
            }
            return DateTime.Now;
        }

        public static DateTime Today()
        {
            return DateTime.Now;
        }

        public static bool DayExists(int day, int month, int year)
        {
            if (day < 1) return false;
            if (month < 1 || month > 12) return false;
            DateTime t = LastDayOfMonth(month, year);
            if (day > t.Day) return false;
            return true;
        }

        public static DateTime LastDayOfMonth(int month, int year)
        {
            if (month < 1 || month > 12) new DateTime();
            DateTime t = new DateTime(year, month, 1);
            t = t.AddMonths(1);
            t = t.AddDays(-1);
            return t;
        }

        public static string StrDate(DateTime t)
        {
            string day = t.Day + "";
            if (day.Length == 1) day = " " + day;
            string month = t.Month + "";
            if (month.Length == 1) month = " " + month;
            return day + "/" + month + "/" + t.Year;
        }

        public static string StrTime(DateTime t)
        {
            string sec = "" + t.Second;
            if (sec.Length == 1) sec = " " + sec;
            string min = "" + t.Minute;
            if (min.Length == 1) min = " " + min;
            string hou = "" + t.Hour;
            if (hou.Length == 1) hou = " " + hou;
            return sec + ":" + min + ":" + hou;
        }

        public static string StrDateTime(DateTime t)
        {
            return StrTime(t) + " - " + StrDate(t);
        }

        public static bool DateTimeFromString(string datetime, out DateTime dt)
        {
            dt = DateTime.Now;
            int[] data = new int[6];
            int index = 0;
            string temp = "";
            for(int i = 0; i < datetime.Length; i++)
            {
                if(datetime[i] == '/' || datetime[i] == ':'
                    || datetime[i] == '-' || datetime[i] == ';')
                {
                    int.TryParse(temp, out data[index]);
                    index++;
                    temp = "";
                    continue;
                }
                temp += datetime[i];
            }
            int.TryParse(temp, out data[index]);
            if (index < 5) return false;
            int[] minValues = new int[] { 0, 0, 0, 1, 1, 1 };
            for (int i = 0; i < 6; i++)
                if (data[i] < minValues[i]) return false;
            int[] maxValues = new int[] { 59, 59, 23, 31, 12, dt.Year + 50};
            for (int i = 0; i < 6; i++)
                if (data[i] > maxValues[i]) return false;
            dt = new DateTime(data[5], data[4], data[3], data[2], data[1], data[0]);
            return true;
        }

        public static bool DateFromString(string date, out DateTime dt)
        {
            dt = DateTime.Now;
            int[] data = new int[3];
            int index = 0;
            string temp = "";
            for (int i = 0; i < date.Length; i++)
            {
                if (date[i] == '/' || date[i] == ':'
                    || date[i] == '-' || date[i] == ';')
                {
                    int.TryParse(temp, out data[index]);
                    index++;
                    temp = "";
                    if (index > 2) return false;
                    continue;
                }
                temp += date[i];
            }
            int.TryParse(temp, out data[index]);
            if (index < 2) return false;
            int[] minValues = new int[] { 1, 1, 1 };
            for (int i = 0; i < 3; i++)
                if (data[i] < minValues[i]) return false;
            int[] maxValues = new int[] { 31, 12, dt.Year + 50 };
            for (int i = 0; i < 3; i++)
                if (data[i] > maxValues[i]) return false;
            dt = new DateTime(data[2], data[1], data[0], 0, 0, 0);
            return true;
        }

        public static bool TimeFromString(string time, out DateTime dt)
        {
            dt = DateTime.Now;
            int[] data = new int[3];
            int index = 0;
            string temp = "";
            for (int i = 0; i < time.Length; i++)
            {
                if (time[i] == '/' || time[i] == ':'
                    || time[i] == '-' || time[i] == ';')
                {
                    int.TryParse(temp, out data[index]);
                    index++;
                    temp = "";
                    if (index > 2) return false;
                    continue;
                }
                temp += time[i];
            }
            int.TryParse(temp, out data[index]);
            if (index < 2) return false;
            int[] minValues = new int[] { 0, 0, 0 };
            for (int i = 0; i < 3; i++)
                if (data[i] < minValues[i]) return false;
            int[] maxValues = new int[] { 59, 59, 23 };
            for (int i = 0; i < 3; i++)
                if (data[i] > maxValues[i]) return false;
            dt = new DateTime(1, 1, 1, data[2], data[1], data[0]);
            return true;
        }

        public static void PrintDate(DateTime t)
        {
            Console.WriteLine(StrDate(t));
        }

        public static void PrintTime(DateTime t)
        {
            Console.WriteLine(StrTime(t));
        }

        public static void PrintDateTime(DateTime t)
        {
            Console.WriteLine(StrDateTime(t));
        }
    }
    
    public struct Card
    {
        public DateTime start;
        public DateTime end;
        public string title;
        public string content;
        public string category;

        public float Begin()
        {
            return Schedule.MinutesToFloat(start);
        }

        public float End()
        {
            return Schedule.MinutesToFloat(end);
        }

        public float Length()
        {
            return End() - Begin();
        }

        public string String()
        {
            return
                "Title: " + title + "\n" 
                + "Start: " + Schedule.StrDateTime(start) + "\n"
                + "End: " + Schedule.StrDateTime(end) + "\n"
                + "Msg: " + content + "\n"
                + "Cat: " + category;
        }
    }

    public struct Deadline
    {
        public DateTime deadline;
        public string title;
        public string category;

        public int SecondsLeft()
        {
            return (int)(deadline - DateTime.Now).TotalSeconds;
        }
    }

    public struct TimeSlot
    {
        public string name;
        public int startSec, startMin, startHou;
        public int endSec, endMin, endHou;

        public TimeSlot(string n, int ss, int sm, int sh, int es, int em, int eh)
        {
            name = n;
            startSec = ss;
            startMin = sm;
            startHou = sh;
            endSec = es;
            endMin = em;
            endHou = eh;
        }
        
        public DateTime StartToDateTime(DateTime dt)
        {
            return new DateTime(dt.Year, dt.Month, dt.Day, startHou, startMin, startSec);
        }

        public DateTime EndToDateTime(DateTime dt)
        {
            return new DateTime(dt.Year, dt.Month, dt.Day, endHou, endMin, endSec);
        }

        public static TimeSlot Read(BinaryReader r)
        {
            string n = r.ReadString();
            int[] res = new int[6];
            for (int i = 0; i < 6; i++)
                res[i] = r.ReadInt32();
            return new TimeSlot(n, res[0], res[1], res[2], res[3], res[4], res[5]);
        }

        public void Write(BinaryWriter w)
        {
            w.Write(name);
            w.Write(startSec);
            w.Write(startMin);
            w.Write(startHou);
            w.Write(endSec);
            w.Write(endMin);
            w.Write(endHou);
        }
    }
}