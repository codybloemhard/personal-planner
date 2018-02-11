using System;
using Trui;

namespace Planner
{
    class Program
    {
        [STAThread]
        static void Main()
        {
            TruiWindow trui = new TruiWindow(1600, 900, Init.init);
        }
    }
}
