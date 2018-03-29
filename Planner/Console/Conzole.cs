using System;

namespace Planner
{
    public static class Conzole//:)
    {
        private static ConsoleColor currentColour = ConsoleColor.White;

        public static void SetDimensions(int w, int h)
        {
            if(Console.BufferWidth < w) Console.BufferWidth = w;
            if (Console.BufferHeight < w) Console.BufferHeight = h;
        }

        public static void SetColour(ConsoleColor col)
        {
            currentColour = col;
            Console.ForegroundColor = currentColour;
        }

        public static void Print(string msg, ConsoleColor col)
        {
            Console.ForegroundColor = col;
            Console.Write(msg);
            Console.ForegroundColor = currentColour;
        }

        public static void PrintLine(string msg, ConsoleColor col)
        {
            Console.ForegroundColor = col;
            Console.WriteLine(msg);
            Console.ForegroundColor = currentColour;
        }

        public static void Print(string msg)
        {
            Console.Write(msg);
        }

        public static void PrintLine(string msg)
        {
            Console.WriteLine(msg);
        }
        
        public static void PrintGrid(char[,] msg)
        {
            for(int y = 0; y < msg.GetLength(1); y++)
            {
                for (int x = 0; x < msg.GetLength(0); x++)
                    Console.Write(msg[x, y]);
            }
        }

        public static string GetLine()
        {
            return Console.ReadLine();
        }

        public static string PadBefore(string s, int l)
        {
            int diff = l - s.Length;
            if (diff == 0) return s;
            string res = "";
            if (diff < 0)
            {
                if (l > 10)
                {
                    res = s.Substring(0, l - 3);
                    res += "...";
                }
                else res = s.Substring(0, l);
                return res;
            }
            for (int i = 0; i < diff; i++)
                res += " ";
            res += s;
            return res;
        }

        public static string PadAfter(string s, int l)
        {
            int diff = l - s.Length;
            if (diff == 0) return s;
            string res = "";
            if (diff < 0)
            {
                if (l > 10)
                {
                    res = s.Substring(0, l - 3);
                    res += "...";
                }
                else res = s.Substring(0, l);
                return res;
            }
            for (int i = 0; i < diff; i++)
                res += " ";
            res = s + res;
            return res;
        }

        public static bool AreYouSure()
        {
            Print("Are you sure? type ", ConsoleColor.Magenta);
            Print("Yes", ConsoleColor.Cyan);
            Print(" or ", ConsoleColor.Magenta);
            Print("No\n", ConsoleColor.Cyan);
            string s = Console.ReadLine();
            s = s.ToLower();
            if (s == "yes") return true;
            if (s == "no") return false;
            PrintLine("Could not detect yes or no", ConsoleColor.Red);
            return AreYouSure();
        }

        public static void Enlist(string[] list)
        {
            for (int i = 0; i < list.Length; i++)
                PrintLine("  - " + list[i]);
        }
    }
}