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
            Drawing.screenSize = drawableSize;

            ColourSchemes.AddScheme("std");
            ColourSchemes.SetScheme("std");
            ColourSchemes.AddColour("std", "dark", Color.FromArgb(255, 32, 32, 32));
            ColourSchemes.AddColour("std", "medium", Color.FromArgb(255, 64, 64, 64));

            scene = new Scene();
            Label menu = new Label(new Space(0f, 0f, 0.2f, 1f), "dark");
            Label button0 = new Label(new Space(0.1f, 0.1f, 0.8f, 0.1f), "medium");
            menu.Add(button0);
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