using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;
using System.Windows.Forms;

namespace Trui
{
    public class Space
    {
        public float x { get; private set; }
        public float y { get; private set; }
        public float w { get; private set; }
        public float h { get; private set; }
        public float padding { get; private set; }
        public bool padXaxis { get; private set; }
        public bool padded { get; private set; }
        public bool inGrid { get; private set; }
        public bool isLeft { get; private set; }
        public bool isRight { get; private set; }
        public bool isTop { get; private set; }
        public bool isBottom { get; private set; }

        public Space(bool full = false)
        {
            if (full) Set(0f, 0f, 1f, 1f);
            else Set(0f, 0f, 0f, 0f);
        }

        public Space(float x, float y, float w, float h)
        {
            Set(x, y, w, h);
        }

        public Space(float padding, bool xAxis, float globalX, float globalY)
        {
            SetPadded(padding, xAxis, globalX, globalY);
        }

        public Space(float padding, bool xAxis, float globalW, float globalH, bool isLeft, bool isRight, bool isTop, bool isBottom)
        {
            SetPaddedGrid(padding, xAxis, globalW, globalH, isLeft, isRight, isTop, isBottom);
        }

        public void Set(float x, float y, float w, float h)
        {
            this.x = x;
            this.y = y;
            this.w = w;
            this.h = h;
            this.padding = 0f;
            this.padded = false;
            Clamp();
        }

        public void RePad(float globalW, float globalH)
        {
            if (!padded) return;
            if (!inGrid) SetPadded(padding, padXaxis, globalW, globalH);
            else SetPaddedGrid(padding, padXaxis, globalW, globalH, isLeft, isRight, isTop, isBottom);
        }

        public void SetPadded(float padding, bool xAxis, float globalW, float globalH)
        {
            Clamp(padding);
            this.padding = padding;
            this.padXaxis = xAxis;
            this.padded = true;
            this.inGrid = false;
            float ratio = xAxis ? globalW / globalH : globalH / globalW;
            if (xAxis)
            {
                x = padding;
                w = 1f - x * 2f;
                y = padding * ratio * Drawing.ratio;
                h = 1f - y * 2f;
            }
            else
            {
                x = padding * ratio / Drawing.ratio;
                w = 1f - x * 2f;
                y = padding;
                h = 1f - y * 2f;
            }
        }

        public void SetPaddedGrid(float padding, bool xAxis, float globalW, float globalH, bool isLeft, bool isRight, bool isTop, bool isBottom)
        {
            Clamp(padding);
            this.padding = padding;
            this.padXaxis = xAxis;
            this.padded = true;
            this.inGrid = true;
            this.isLeft = isLeft;
            this.isRight = isRight;
            this.isTop = isTop;
            this.isBottom = isBottom;
            float ratio = xAxis ? globalW / globalH : globalH / globalW;
            if (xAxis)
            {
                float verpad = padding * ratio * Drawing.ratio;
                x = isLeft ? padding : padding / 2f;
                w = isRight ? 1f - x - padding : 1f - x - (padding / 2f);
                y = isTop ? verpad : verpad / 2f;
                h = isBottom ? 1f - y - verpad : 1f - y - (verpad / 2f);
            }
            else
            {
                float horpad = padding * ratio / Drawing.ratio;
                x = isLeft ? horpad : horpad / 2f;
                w = isRight ? 1f - x - horpad : 1f - x - (horpad / 2f);
                y = isTop ? padding : padding / 2f;
                h = isBottom ? 1f - y - padding : 1f - y - (padding / 2f);
            }
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

        public string String()
        {
            return "" + x + " - " + y + " - " + w + " - " + h;
        }
    }

    public struct MouseEvent
    {
        public MouseButtons button;
        public bool clicked, down;
        public float x, y;
    }

    public class UINode
    {
        public UINode parent { get; private set; }
        protected Space space { get; private set; }
        protected Space globalSpace { get; private set; }
        private List<UINode> childs;

        public UINode(Space space)
        {
            childs = new List<UINode>();
            SetSpace(space);
        }

        public void Clear()
        {
            childs.Clear();
        }

        public void Add(UINode node)
        {
            node.parent = this;
            childs.Add(node);
            node.ComputeGlobalSpace();
        }

        public void AddPadded(UINode node, float padding, bool xAxis)
        {
            node.parent = this;
            node.space = new Space(padding, xAxis, globalSpace.w, globalSpace.h);
            childs.Add(node);
            node.ComputeGlobalSpace();
        }

        public void AddPaddedGridStyle(UINode node, float padding, bool xAxis, bool isLeft, bool isRight, bool isTop, bool isBottom)
        {
            node.parent = this;
            node.space = new Space(padding, xAxis, globalSpace.w, globalSpace.h, isLeft, isRight, isTop, isBottom);
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

        public virtual void MouseEvent(MouseEvent e)
        {
            for (int i = 0; i < childs.Count; i++)
                childs[i].MouseEvent(e);
        }

        protected bool IsInsideSpace(float x, float y)
        {
            if (x > globalSpace.x && x < globalSpace.x + globalSpace.w
                && y > globalSpace.y && y < globalSpace.y + globalSpace.h)
                return true;
            return false;
        }

        private void ComputeGlobalSpace()
        {
            if (space.padded)
                space.RePad(parent.globalSpace.w, parent.globalSpace.h);
            if (parent == null)
                globalSpace = new Space(0f, 0f, 1f, 1f);
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

    public class Grid : UINode
    {
        private UINode[,] grid;
        private uint w, h;

        public Grid(Space space, uint w, uint h) : base(space)
        {
            grid = new UINode[w, h];
            this.w = w;
            this.h = h;
            CreateGrid(w, h);
        }

        private void CreateGrid(uint w, uint h)
        {
            if (w < 1) w = 1;
            if (h < 1) h = 1;
            if (w * h == 1) return;
            float stepx = 1f / w;
            float stepy = 1f / h; 
            for (int x = 0; x < w; x++)
                for (int y = 0; y < h; y++)
                {
                    float xx = stepx * x;
                    float yy = stepy * y;
                    Space gridpiece = new Space(xx, yy, stepx, stepy);
                    UINode gridelement = new UINode(gridpiece);
                    grid[x, y] = gridelement;
                    Add(gridelement);
                }
        }

        public void Add(UINode node, uint x, uint y)
        {
            if (x >= h && y >= h) return;
            grid[x, y].Add(node);
        }

        public void AddPadded(UINode node, float padding, bool xAxis, uint x, uint y)
        {
            if (x >= h && y >= h) return;
            grid[x, y].AddPadded(node, padding, xAxis);
        }

        public void AddPaddedEven(UINode node, float padding, bool xAxis, uint x, uint y)
        {
            if (x >= h && y >= h) return;
            bool isLeft = x == 0;
            bool isRight = x == w - 1;
            bool isTop = y == 0;
            bool isBottom = y == h - 1;
            grid[x, y].AddPaddedGridStyle(node, padding, xAxis, isLeft, isRight, isTop, isBottom);
        }

        public void Remove(UINode node, uint x, uint y)
        {
            if (x >= h && y >= h) return;
            grid[x, y].Remove(node);
        }
    }

    public class Drawable : UINode
    {
        public string colour { get; protected set; }
        protected Rectangle screenPart;

        public Drawable(Space space, string colour)
            : base(space)
        {
            this.colour = colour;
        }

        protected void UpdateScreenPart()
        {
            int x = (int)Math.Round(globalSpace.x * Drawing.screenSize.Width);
            int y = (int)Math.Round(globalSpace.y * Drawing.screenSize.Height);
            int w = (int)Math.Round(globalSpace.w * Drawing.screenSize.Width);
            int h = (int)Math.Round(globalSpace.h * Drawing.screenSize.Height);
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

    public class Button : Label
    {
        protected string baseColour, hoverColour, pressColour;
        protected string textColour, font, text;
        protected Action action;

        public Button(Space space, Action action, string baseColour, string hoverColour, string pressColour,
            string textColour, string text, string font)
            : base(space, baseColour)
        {
            this.baseColour = baseColour;
            this.hoverColour = hoverColour;
            this.pressColour = pressColour;
            this.textColour = textColour;
            this.action = action;
            this.font = font;
            this.text = text;
            TextLine line = new TextLine(new Space(true), text, font, textColour);
            this.Add(line);
        }

        public override void MouseEvent(MouseEvent e)
        {
            bool hit = base.IsInsideSpace(e.x, e.y);
            string oldColour = colour;
            if (!hit)
                colour = baseColour;
            else if (hit && e.clicked && e.button == MouseButtons.Left)
                action();
            else if (hit && e.down && e.button == MouseButtons.Left)
                colour = pressColour;
            else if (hit) colour = hoverColour;

            if (oldColour != colour)
                Drawing.needRedraw = true;
        }
    }

    public class TextLine : Drawable
    {
        public string text { get; protected set; }
        public string font { get; protected set; }
        public TextFormatFlags format { get; set; }

         public TextLine(Space space, string text, string font, string colour) 
            : base(space, colour)
        {
            this.text = text;
            this.font = font;
            this.format = TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter;
        }

        public override void Draw(Graphics g)
        {
            base.Draw(g);
            SchemeColour col = ColourSchemes.GetColour(colour);
            Color c = col.colour;
            Font f = Fonts.Get(font);
            TextRenderer.DrawText(g, text, f, this.screenPart, c, format);
        }
    }
}