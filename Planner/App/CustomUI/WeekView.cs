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
            DateTime first = MyCalendar.FirstDayOfTheWeek(DateTime.Now);
            string[] dayz = new string[] { "MA", "DI", "WO", "DO", "VR", "ZA", "ZO"};
            int currentDayNr = 0;
            for (int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "light" : "medium";
                Label l = new Label(sp, colour);
                string msg = dayz[i] + " - " + first.Day;
                bool isToday = first.Day == DateTime.Now.Day;
                if (isToday) currentDayNr = i;
                string textColour = isToday ? "green" : "red";
                first = first.AddDays(1);
                TextLine line = new TextLine(new Space(true), msg, "daysInWeek", textColour);
                l.Add(line);
                daysSpace.Add(l);
            }
            //build calendar
            Label[] dayLabels = new Label[7];
            for(int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "medium" : "dark";
                Label l = new Label(sp, colour);
                calendarSpace.Add(l);
                dayLabels[i] = l;
            }
            //add cards
            for (int i = 0; i < MyCalendar.Cards(); i++)
            {
                Card currentCard = MyCalendar.GetCard(i);
                bool thisWeek = MyCalendar.IsInThisWeek(first, currentCard.start);
                if (!thisWeek) continue;
                Label card = new Label(new Space(0f, currentCard.Begin(), 1f, currentCard.Length()), "darkblue");
                TextLine cardLine = new TextLine(new Space(0f, 0f, 1f, 0.25f), currentCard.title, "cardTitle", "red");
                card.Add(cardLine);
                int index = MyCalendar.DaySinceMonday(currentCard.start);
                dayLabels[Math.Abs(index)].Add(card);
            }
            //build timebar
            float part = MyCalendar.MinutesToFloat(DateTime.Now);
            float height = 0.01f;
            Label timeLine = new Label(new Space(0f, part - height/2f, 1f, height),"red");
            dayLabels[currentDayNr].Add(timeLine);
        }
    }
}