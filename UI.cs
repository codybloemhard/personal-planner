using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;
using System.Windows.Forms;

namespace Planner
{
    public class Space
    {
        public float x { get; private set; }
        public float y { get; private set; }
        public float w { get; private set; }
        public float h { get; private set; }

        public Space(float x, float y, float w, float h)
        {
            Set(x, y, w, h);
        }

        public void Set(float x, float y, float w, float h)
        {
            this.x = x;
            this.y = y;
            this.w = w;
            this.h = h;
            Clamp();
        }

        private void Clamp()
        {
            x = Clamp(x);
            y = Clamp(y);
            w = Clamp(w);
            h = Clamp(h);
        }

        private float Clamp(float x)
        {
            if (x < 0f) x = 0f;
            if (x > 1f) x = 1f;
            return x;
        }
    }

    public class UINode
    {
        public string colour { get; protected set; }
        public UINode parent { get; private set; }
        protected Space space { get; private set; }
        protected Space globalSpace { get; private set; }
        private List<UINode> childs;

        public UINode(Space space, string colour)
        {
            childs = new List<UINode>();
            this.colour = colour;
            SetSpace(space);
        }

        public void Add(UINode node)
        {
            node.parent = this;
            childs.Add(node);
            node.ComputeGlobalSpace();
        }

        public void Remove(UINode node)
        {
            node.parent = null;
            if(childs.Contains(node))
                childs.Remove(node);
        }

        public void Destroy()
        {
            if (parent == null) return;
            parent.childs.Remove(this);
        }

        public virtual void DrawAllUnder(Graphics g)
        {
            for (int i = 0; i < childs.Count; i++)
                childs[i].DrawAllUnder(g);
        }

        public void SetSpace(Space space)
        {
            this.space = space;
            ComputeGlobalSpace();
        }

        private void ComputeGlobalSpace()
        {
            if(parent == null)
            {
                globalSpace = new Space(0f, 0f, 1f, 1f);
            }
            else
            {
                float x = parent.globalSpace.x + (parent.globalSpace.w * space.x);
                float y = parent.globalSpace.y + (parent.globalSpace.h * space.y);
                float w = parent.globalSpace.w * space.w;
                float h = parent.globalSpace.h * space.h;
                globalSpace = new Space(x, y, w, h);
            }
            for (int i = 0; i < childs.Count; i++)
                childs[i].ComputeGlobalSpace();
        }
    }

    public class Drawable : UINode
    {
        protected Rectangle screenPart;

        public Drawable(Space space, string colour)
            : base(space, colour) { }

        protected void UpdateScreenPart()
        {
            int x = (int)(globalSpace.x * Drawing.screenSize.Width);
            int y = (int)(globalSpace.y * Drawing.screenSize.Height);
            int w = (int)(globalSpace.w * Drawing.screenSize.Width);
            int h = (int)(globalSpace.h * Drawing.screenSize.Height);
            screenPart = new Rectangle(x, y, w, h);
        }

        public override void DrawAllUnder(Graphics g)
        {
            Draw(g);
            base.DrawAllUnder(g);
        }

        public virtual void Draw(Graphics g)
        {
            UpdateScreenPart();
        }
    }

    public class Label : Drawable
    {
        public Label(Space space, string colour)
            : base(space, colour) { }

        public override void Draw(Graphics g)
        {
            base.Draw(g);
            Drawing.DrawRectangle(g, screenPart, colour);
        }
    }
}