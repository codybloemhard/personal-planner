using System;
using System.ComponentModel;
using System.Drawing;
using System.Windows.Forms;

namespace Trui
{
    public class TruiWindow : Form
    {
        public TruiWindow(uint w, uint h, Action init)
        {
            Size drawableSize = new Size((int)w, (int)h);
            this.ClientSize = drawableSize;
            Drawing.SetScreen(drawableSize);
            this.ResizeRedraw = true;
            init();
            Application.EnableVisualStyles();
            Application.Run(this);
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            Scenes.DrawAll(e.Graphics);
            Drawing.needRedraw = false;
        }

        protected override void OnResize(EventArgs e)
        {
            base.OnResize(e);
            int w = ClientSize.Width;
            float aspect = 16f / 9f;
            this.ClientSize = new Size(w, (int)(w / aspect));
            Drawing.SetScreen(ClientSize);
            Fonts.Recalculate();
        }

        protected override void OnMouseClick(MouseEventArgs e)
        {
            base.OnMouseClick(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = true;
            ev.down = false;
            ev.button = e.Button;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }

        protected override void OnMouseDown(MouseEventArgs e)
        {
            base.OnMouseDown(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = false;
            ev.down = true;
            ev.button = e.Button;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }

        protected override void OnMouseMove(MouseEventArgs e)
        {
            base.OnMouseMove(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = false;
            ev.button = MouseButtons.None;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }
    }
}