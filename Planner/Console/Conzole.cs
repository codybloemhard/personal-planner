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
    }
}