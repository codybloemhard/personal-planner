using System;
using System.Threading;
using Trui;
using Planner;

namespace Planner
{
    class Program
    {
        [STAThread]
        static void Main()
        {
            MyCalendar.InitSchedule();
            TruiWindow trui = new TruiWindow(1600, 900, Init.init);
        }
    }
}
