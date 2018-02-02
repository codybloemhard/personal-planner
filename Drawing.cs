using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Drawing;

namespace Planner
{
    public class SchemeColour
    {
        public Color colour;
        public SolidBrush brush;
    }

    public static class Drawing
    {
        private static Dictionary<string, SchemeColour> scheme;

        static Drawing()
        {
            scheme = new Dictionary<string, SchemeColour>();
        }

        public static void AddColour(Color c)
        {
            SchemeColour colour = new SchemeColour();
            colour.colour = c;
            colour.brush = new SolidBrush(c);
        }

        public static void Draw(PaintEventArgs e)
        {
            
        }

        public static void DrawRectangle(Graphics g, Rectangle r, string schemeColour)
        {
            if (!scheme.ContainsKey(schemeColour)) return;
            g.FillRectangle(scheme[schemeColour].brush, r);
        }
    }
}