using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;

namespace Planner
{
    public static class MyCalendar
    {
        private static List<Card> cards;
        private static Card nullcard;

        static MyCalendar()
        {
            cards = new List<Card>();
            nullcard = new Card();
        }

        public static int Cards()
        {
            return cards.Count;
        }

        public static Card GetCard(int i)
        {
            if (i < 0 || i > cards.Count - 1) return nullcard;
            return cards[i];
        }

        public static void InitSchedule()
        {
            string file = "agendaData";
            if (File.Exists(file))
                LoadData(file);
            for (int i = 0; i < cards.Count; i++)
                Console.WriteLine(cards[i].String());

            Card c0 = new Card();
            c0.start = new DateTime(2018, 2, 13, 18, 0, 0);
            c0.end = new DateTime(2018, 2, 13, 21, 30, 0);
            c0.title = "Databases homework";
            c0.content = "do shit on blackboard ok";
            c0.category = "uni";
            Card c1 = new Card();
            c1.start = new DateTime(2018, 2, 16, 12, 0, 0);
            c1.end = new DateTime(2018, 2, 16, 13, 0, 0);
            c1.title = "Eten";
            c1.content = "hap hap hap";
            c1.category = "misc";
            cards.Add(c0);
            cards.Add(c1);
            WriteData(file);
        }
        
        private static void LoadData(string file)
        {
            BinaryReader r = new BinaryReader(File.Open(file, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Card c = new Card();
                c.start = ReadDateTime(r);
                c.end = ReadDateTime(r);
                c.title = r.ReadString();
                c.content = r.ReadString();
                c.category = r.ReadString();
            }
            r.Close();
        }

        private static void WriteData(string file)
        {
            BinaryWriter w = new BinaryWriter(File.Open(file, FileMode.OpenOrCreate));
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

        private static DateTime ReadDateTime(BinaryReader r)
        {
            int[] datetime = new int[6];
            for (int j = 0; j < 6; j++)
            {
                datetime[j] = r.ReadInt32();
                Console.Write(datetime[j] + " - ");
            }
            Console.WriteLine();
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
            return t.Hour + "/" + t.Minute + "/" + t.Second;
        }

        public static string StrDateTime(DateTime t)
        {
            return StrDate(t) + "::" + StrTime(t);
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
            return MyCalendar.MinutesToFloat(start);
        }

        public float End()
        {
            return MyCalendar.MinutesToFloat(end);
        }

        public float Length()
        {
            return End() - Begin();
        }

        public string String()
        {
            return
                "Title: " + title + "\n" 
                + "Start: " + MyCalendar.StrDateTime(start) + "\n"
                + "End: " + MyCalendar.StrDateTime(end) + "\n"
                + "Msg: " + content + "\n"
                + "Cat: " + category;
        }
    }
}