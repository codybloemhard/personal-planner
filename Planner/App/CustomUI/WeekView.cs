using System;
using Trui;

namespace Planner
{
    public class WeekView : UINode
    {
        public WeekView(Space space) 
            : base(space)
        {
            UINode agendaSpace = new UINode(new Space(0.1f, 0f, 0.9f, 1f));
            UINode menuSpace = new UINode(new Space(0f, 0f, 0.1f, 1f));
            this.Add(agendaSpace);
            this.Add(menuSpace);
            UINode daysSpace = new UINode(new Space(0f, 0f, 1f, 0.05f));
            UINode calendarSpace = new UINode(new Space(0f, 0.05f, 1f, 0.95f));
            agendaSpace.Add(daysSpace);
            agendaSpace.Add(calendarSpace);
            float step = 1f / 7f;
            //build days labels
            string[] dayz = new string[] { "MA", "DI", "WO", "DO", "VR", "ZA", "ZO"};
            for(int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "light" : "medium";
                Label l = new Label(sp, colour);
                string msg = dayz[i] + " - " + DateTime.Now.Day;
                TextLine line = new TextLine(new Space(true), msg, "daysInWeek", "red");
                l.Add(line);
                daysSpace.Add(l);
            }
            //build calendar
            for(int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "medium" : "dark";
                Label l = new Label(sp, colour);
                calendarSpace.Add(l);
            }
            MyCalendar.Test();
        }
    }
}