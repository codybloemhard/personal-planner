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
            for(int i = 0; i < 100; i++)
            {
                time = time.AddDays(1);
                Print(time);
            }
            Console.Write("Last: ");
            Console.WriteLine(DayExists(32, 3, 2018));
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
}