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
            
            WeekView weekView = new WeekView(new Space(0f, 0.1f, 1f, 0.9f));       
            Scenes.Add(weekView);
        }
    }
}