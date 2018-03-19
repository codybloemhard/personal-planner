using System;

namespace Planner
{
    public static class Conzole//:)
    {
        public static void Print(string msg, ConsoleColor col)
        {
            Console.ForegroundColor = col;
            Console.Write(msg);
        }

        public static void PrintLine(string msg, ConsoleColor col)
        {
            Console.ForegroundColor = col;
            Console.WriteLine(msg);
        }

        public static string GetLine()
        {
            return Console.ReadLine();
        }
    }
}