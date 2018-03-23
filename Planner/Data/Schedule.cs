using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;

namespace Planner
{
    public static class Schedule
    {
        private static List<Card> cards;
        private static Card nullcard;
        private static List<Deadline> deadlines;
        private static Deadline nulldeadline;

        private static string cardFile = "agendaData";
        private static string deadlineFile = "deadlineData";

        static Schedule()
        {
            cards = new List<Card>();
            nullcard = new Card();
            deadlines = new List<Deadline>();
            nulldeadline = new Deadline();
        }

        public static int AmountCards()
        {
            return cards.Count;
        }

        public static Card GetCard(int i)
        {
            if (i < 0 || i > cards.Count - 1) return nullcard;
            return cards[i];
        }

        public static int AmountDeadlines()
        {
            return deadlines.Count;
        }

        public static Deadline GetDeadline(int i)
        {
            if (i < 0 || i > deadlines.Count - 1) return nulldeadline;
            return deadlines[i];
        }

        public static void InitSchedule()
        {
            LoadCards();
            LoadDeadlines();
        }

        public static void LoadCards()
        {
            if (!File.Exists(cardFile)) return;
            BinaryReader r = new BinaryReader(File.Open(cardFile, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Card c = new Card();
                c.start = ReadDateTime(r);
                c.end = ReadDateTime(r);
                c.title = r.ReadString();
                c.content = r.ReadString();
                c.category = r.ReadString();
                cards.Add(c);
            }
            r.Close();
        }

        public static void WriteCards()
        {
            BinaryWriter w = new BinaryWriter(File.Open(cardFile, FileMode.OpenOrCreate));
            w.Write(cards.Count);
            for (int i = 0; i < cards.Count; i++)
            {
                WriteDateTime(w, cards[i].start);
                WriteDateTime(w, cards[i].end);
                w.Write(cards[i].title);
                w.Write(cards[i].content);
                w.Write(cards[i].category);
            }
            w.Close();
        }

        public static void LoadDeadlines()
        {
            if (!File.Exists(deadlineFile)) return;
            BinaryReader r = new BinaryReader(File.Open(deadlineFile, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Deadline d = new Deadline();
                d.deadline = ReadDateTime(r);
                d.title = r.ReadString();
                d.category = r.ReadString();
                deadlines.Add(d);
            }
            r.Close();
        }

        public static void WriteDeadlines()
        {
            BinaryWriter w = new BinaryWriter(File.Open(deadlineFile, FileMode.OpenOrCreate));
            w.Write(deadlines.Count);
            for (int i = 0; i < deadlines.Count; i++)
            {
                WriteDateTime(w, deadlines[i].deadline);
                w.Write(deadlines[i].title);
                w.Write(deadlines[i].category);
            }
            w.Close();
        }

        public static void AddDeadline(Deadline l)
        {
            deadlines.Add(l);
        }

        public static void DeleteDeadline(Deadline l)
        {
            deadlines.Remove(l);
        }

        private static DateTime ReadDateTime(BinaryReader r)
        {
            int[] datetime = new int[6];
            for (int j = 0; j < 6; j++)
                datetime[j] = r.ReadInt32();
            DateTime dt = new DateTime(datetime[0], datetime[1], datetime[2], 
                datetime[3], datetime[4], datetime[5]);
            return dt;
        }

        private static void WriteDateTime(BinaryWriter w, DateTime t)
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
            return t.Day + "/" + t.Month + "/" + t.Year;
        }

        public static string StrTime(DateTime t)
        {
            return t.Second + ":" + t.Minute + ":" + t.Hour;
        }

        public static string StrDateTime(DateTime t)
        {
            return StrTime(t) + "::" + StrDate(t);
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
                    || datetime[i] == '-')
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
            for (int i = 0; i < 6; i++)
                if (data[i] < 0) return false;
            int[] maxValues = new int[] { 59, 59, 23, 31, 12, dt.Year + 50};
            for (int i = 0; i < 6; i++)
                if (data[i] > maxValues[i]) return false;
            dt = new DateTime(data[5], data[4], data[3], data[2], data[1], data[0]);
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

        public int SecondsUntil()
        {
            return (int)(deadline - DateTime.Now).TotalSeconds;
        }
    }
}