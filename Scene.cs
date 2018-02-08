using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;

namespace Planner
{
    public class Scene
    {
        private Label root;
        
        public Scene()
        {
            root = new Label(new Space(0f, 0f, 1f, 1f), "black");
        }

        public void Add(UINode newNode)
        {
            root.Add(newNode);
        }

        public void DrawAll(Graphics g)
        {
            root.DrawAllUnder(g);
        }

        public void FeedMouseEvent(MouseEvent e)
        {
            root.MouseEvent(e);
        }
    }
}