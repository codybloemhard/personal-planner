using System;
using Trui;
using System.Drawing;

namespace Planner
{
    public static class Init
    {
        public static void init()
        {
            ColourSchemes.Add("std");
            ColourSchemes.Set("std");
            ColourSchemes.Add("std", "dark", Color.FromArgb(255, 32, 32, 32));
            ColourSchemes.Add("std", "medium", Color.FromArgb(255, 64, 64, 64));
            ColourSchemes.Add("std", "light", Color.FromArgb(255, 128, 128, 128));
            ColourSchemes.Add("std", "green", Color.Green);
            ColourSchemes.Add("std", "red", Color.Red);
            ColourSchemes.Add("std", "darkblue", Color.DarkBlue);

            Fonts.Add("cardTitle", 0.01f);
            Fonts.Add("daysInWeek", 0.02f, FontStyle.Bold | FontStyle.Italic);
            Fonts.Add("medium", 0.05f);
            Fonts.Add("bold", 0.05f, FontStyle.Bold);
            Fonts.Recalculate();

            Agenda.Init();
            Scenes.Set("agenda");
        }
    }
}