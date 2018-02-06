using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace Planner
{
    public partial class MainForm : Form
    {
        private Scene scene;

        public MainForm()
        {
            InitializeComponent();
            Size drawableSize = new Size(1600, 900);
            this.ClientSize = drawableSize;
            Drawing.SetScreen(drawableSize);

            ColourSchemes.AddScheme("std");
            ColourSchemes.SetScheme("std");
            ColourSchemes.AddColour("std", "dark", Color.FromArgb(255, 32, 32, 32));
            ColourSchemes.AddColour("std", "medium", Color.FromArgb(255, 64, 64, 64));
            ColourSchemes.AddColour("std", "light", Color.FromArgb(255, 128, 128, 128));
            
            scene = new Scene();
            Label menu = new Label(new Space(0f, 0f, 0.2f, 1f), "dark");
            Label test = new Label(new Space(), "medium");
            uint w = 4, h = 3;
            Grid grid = new Grid(new Space(true), w, h);
            for (int x = 0; x < w; x++)
                for (int y = 0; y < h; y++)
                {
                    Label l = new Label(new Space(), "medium");
                    grid.AddPaddedEven(l, 0.1f, true, (uint)x, (uint)y);
                }
            menu.Add(grid);
            scene.Add(menu);        
        }
        
        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            Drawing.Draw(e);
            scene.DrawAll(e.Graphics);
        }
    }
}