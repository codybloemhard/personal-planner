using System;
using System.Collections.Generic;

namespace Planner
{
    public static class Logic
    {
        public static DateTime Limit(string s)
        {
            DateTime limit = DateTime.MaxValue;
            DateTime n = DateTime.Now;
            if (s == "today")
                limit = new DateTime(n.Year, n.Month, n.Day);
            else if (s == "day")
                limit = n.AddDays(1);
            else if (s == "thisweek")
                limit = Schedule.FirstDayOfTheWeek(n).AddDays(6);
            else if (s == "week")
                limit = n.AddDays(7);
            else if (s == "thismonth")
                limit = Schedule.LastDayOfMonth(n.Month, n.Year);
            else if (s == "month")
                limit = n.AddMonths(1);
            else if (s == "thisyear")
                limit = Schedule.LastDayOfMonth(12, n.Year);
            else if (s == "year")
                limit = n.AddYears(1);
            else return limit;
            limit = new DateTime(limit.Year, limit.Month, limit.Day, 23, 59, 59);
            return limit;
        }
    }
}