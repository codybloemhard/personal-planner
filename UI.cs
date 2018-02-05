﻿using System;
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
        public float padding { get; private set; }
        public bool padXaxis { get; private set; }
        public bool padded { get; private set; }

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
            SetPadded(padding, padXaxis, globalW, globalH);
        }

        public void SetPadded(float padding, bool xAxis, float globalW, float globalH)
        {
            Clamp(padding);
            this.padding = padding;
            this.padXaxis = xAxis;
            this.padded = true;
            float ratio = xAxis ? globalW / globalH : globalH / globalW;
            if (xAxis)
            {
                x = padding;
                w = 1f - x*2f;
                y = padding * ratio * Drawing.ratio;
                h = 1f - y*2f;
            }
            else
            {
                x = padding * ratio / Drawing.ratio;
                w = 1f - x*2f;
                y = padding;
                h = 1f - y*2f;
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