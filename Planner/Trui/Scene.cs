using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;

namespace Trui
{
    public static class Scenes
    {
        private static Dictionary<string, Scene> scenes;
        private static Scene current;

        static Scenes()
        {
            scenes = new Dictionary<string, Scene>();
            current = null;
        }

        public static void Add(string name)
        {
            if (scenes.ContainsKey(name)) return;
            scenes.Add(name, new Scene());
        }

        public static Scene Get(string name)
        {
            if (!scenes.ContainsKey(name)) return null;
            return scenes[name];
        }

        public static void Set(string name)
        {
            if (!scenes.ContainsKey(name)) return;
            current = scenes[name];
        }

        public static void Add(UINode node)
        {
            if (current == null) return;
            current.Add(node);
        }

        public static void DrawAll(Graphics g)
        {
            if (current == null) return;
            current.DrawAll(g);
        }

        public static void FeedMouseEvent(MouseEvent e)
        {
            if (current == null) return;
            current.FeedMouseEvent(e);
        }
    }

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