using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;

namespace Planner
{
    public static class MyCalendar
    {
        private static List<Card> cards;

        static MyCalendar()
        {
            cards = new List<Card>();
            string file = "agendaData";
            if (File.Exists(file))
                LoadData(file);
        }
        
        private static void LoadData(string file)
        {
            BinaryReader r = new BinaryReader(File.Open(file, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Card c = new Card();
                c.begin = ReadDateTime(r);
                c.end = ReadDateTime(r);
                c.title = r.ReadString();
                c.content = r.ReadString();
                c.category = r.ReadString();
            }
        }

        private static void WriteData(string file)
        {
            BinaryWriter w = new BinaryWriter(File.Open(file, FileMode.OpenOrCreate));
            for(int i = 0; i < cards.Count; i++)
            {
                WriteDateTime(w, cards[i].begin);
                WriteDateTime(w, cards[i].end);
                w.Write(cards[i].title);
                w.Write(cards[i].content);
                w.Write(cards[i].category);
            }
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

        public static void Test()
        {
            DateTime time = DateTime.Now;
            Console.Write("Today: "); Print(time);
            Print(FirstDayOfTheWeek(time));
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

        public static void Print(DateTime t)
        {
            Console.WriteLine(t.Day + "/" + t.Month + "/" + t.Year);
        }
    }

    public struct Card
    {
        public DateTime begin;
        public DateTime end;
        public string title;
        public string content;
        public string category;

        public float Begin()
        {
            return MyCalendar.MinutesToFloat(begin);
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
            
        }
    }
}