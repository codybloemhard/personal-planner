using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;
using System.Windows.Forms;

namespace Planner
{
    public class Label
    {
        public Rectangle space { get; protected set; }
        public string colour { get; protected set; }

        public Label(Rectangle size, string colour)
        {
            this.space = size;
            this.colour = colour;
        }

        public void Draw(Graphics g)
        {
            Drawing.DrawRectangle(g, space, colour);
        }
    }
}