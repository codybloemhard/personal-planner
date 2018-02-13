using System;
using System.Collections.Generic;
using System.Globalization;

namespace Planner
{
    public static class MyCalendar
    {
        static MyCalendar()
        {
            
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
    }
}