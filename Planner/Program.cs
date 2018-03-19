using System;
using System.Threading;
using Planner;

namespace Planner
{
    class Program
    {
        [STAThread]
        static void Main()
        {
            MyCalendar.InitSchedule();
            Interperter main = new Interperter();
        }
    }
}