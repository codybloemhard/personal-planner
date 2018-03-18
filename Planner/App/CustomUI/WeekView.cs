using System;
using Trui;

namespace Planner
{
    public class WeekView : UINode
    {
        private UINode agendaSpace, menuSpace, daysSpace, calendarSpace;
        private Label[] dayLabels;
        private float step = 1f / 7f;
        private DateTime currentWeek;

        public WeekView(Space space) 
            : base(space)
        {
            agendaSpace = new UINode(new Space(0.1f, 0f, 0.9f, 1f));
            menuSpace = new UINode(new Space(0f, 0f, 0.1f, 1f));
            this.Add(agendaSpace);
            this.Add(menuSpace);
            daysSpace = new UINode(new Space(0f, 0f, 1f, 0.05f));
            calendarSpace = new UINode(new Space(0f, 0.05f, 1f, 0.95f));
            agendaSpace.Add(daysSpace);
            agendaSpace.Add(calendarSpace);
            //build calendar
            dayLabels = new Label[7];
            for (int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "medium" : "dark";
                Label l = new Label(sp, colour);
                calendarSpace.Add(l);
                dayLabels[i] = l;
            }
            //setup buttons
            Button nextWeekButton = new Button(new Space(0f, 0f, 1f, 0.2f), NextWeekAction, "medium", "light", "dark", "red", "Next Week", "cardTitle");
            Button prevWeekButton = new Button(new Space(0f, 0.2f, 1f, 0.2f), PrevWeekAction, "medium", "light", "dark", "red", "Prev Week", "cardTitle");
            menuSpace.Add(nextWeekButton);
            menuSpace.Add(prevWeekButton);
            //finilize
            currentWeek = DateTime.Now;
            Show(currentWeek);
        }

        public void Show(DateTime firstDay)
        {
            //clear shit up
            daysSpace.Clear();
            for (int i = 0; i < 7; i++)
                dayLabels[i].Clear();
            //build days labels
            DateTime first;
            if (firstDay.DayOfWeek == DayOfWeek.Monday)
                first = firstDay;
            else first = MyCalendar.FirstDayOfTheWeek(firstDay);
            bool currentWeekIsRealCurrentWeek = MyCalendar.IsInThisWeek(first, MyCalendar.FirstDayOfTheWeek(DateTime.Now));
            string[] dayz = new string[] { "MA", "DI", "WO", "DO", "VR", "ZA", "ZO" };
            int currentDayNr = 0;
            for (int i = 0; i < 7; i++)
            {
                Space sp = new Space(step * i, 0f, step, 1f);
                string colour = i % 2 == 0 ? "light" : "medium";
                Label l = new Label(sp, colour);
                string msg = dayz[i] + " - " + first.Day;
                bool isToday = first.Day == DateTime.Now.Day;
                isToday = isToday && currentWeekIsRealCurrentWeek;
                if (isToday) currentDayNr = i;
                string textColour = isToday ? "green" : "red";
                first = first.AddDays(1);
                TextLine line = new TextLine(new Space(true), msg, "daysInWeek", textColour);
                l.Add(line);
                daysSpace.Add(l);
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
            if (currentWeekIsRealCurrentWeek)
            {
                float part = MyCalendar.MinutesToFloat(DateTime.Now);
                float height = 0.01f;
                Label timeLine = new Label(new Space(0f, part - height / 2f, 1f, height), "red");
                dayLabels[currentDayNr].Add(timeLine);
            }
            Drawing.needRedraw = true;
        }

        private void NextWeekAction()
        {
            currentWeek = MyCalendar.NextDay(MyCalendar.FirstDayOfTheWeek(currentWeek), DayOfWeek.Monday);
            Show(currentWeek);
        }

        private void PrevWeekAction()
        {
            currentWeek = MyCalendar.PrevDay(MyCalendar.FirstDayOfTheWeek(currentWeek), DayOfWeek.Monday);
            Show(currentWeek);
        }
    }
}