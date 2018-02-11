using System;
using Trui;

namespace Planner
{
    public static class Agenda
    {
        public static void Init()
        {
            Scenes.Add("agenda");
            Scenes.Set("agenda");

            Label menu = new Label(new Space(0f, 0f, 0.2f, 1f), "dark");
            Label test = new Label(new Space(), "medium");
            uint w = 5, h = 3;
            Grid grid = new Grid(new Space(true), w, h);
            for (int x = 0; x < w; x++)
                for (int y = 0; y < h; y++)
                {
                    Label l = new Label(new Space(), "medium");
                    grid.AddPaddedEven(l, 0.1f, true, (uint)x, (uint)y);
                }
            menu.Add(grid);
            Scenes.Add(menu);

            Button button = new Button(new Space(0.3f, 0.1f, 0.4f, 0.1f), () => { Console.WriteLine("hello"); }, "medium", "light", "green",
                "red", "click me boi", "medium");
            Scenes.Add(button);
        }
    }
}