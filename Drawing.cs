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

        public SchemeColour(Color colour)
        {
            this.colour = colour;
            brush = new SolidBrush(this.colour);
        }
    }

    public class ColourScheme
    {
        private Dictionary<string, SchemeColour> scheme;

        public ColourScheme()
        {
            scheme = new Dictionary<string, SchemeColour>();
        }

        public void AddColour(string name, Color c)
        {
            SchemeColour colour = new SchemeColour(c);
            scheme.Add(name, colour);
        }

        public SchemeColour GetColour(string colour)
        {
            if (!scheme.ContainsKey(colour)) return null;
            return scheme[colour];
        }
    }

    public static class ColourSchemes
    {
        private static Dictionary<string, ColourScheme> schemes;
        private static string currentScheme;
        private static SchemeColour black, white;

        static ColourSchemes()
        {
            schemes = new Dictionary<string, Planner.ColourScheme>();
            currentScheme = "";
            black = new SchemeColour(Color.Black);
            white = new SchemeColour(Color.White);
        }

        public static void Add(string name)
        {
            if (schemes.ContainsKey(name)) return;
            schemes.Add(name, new ColourScheme());
        }

        public static void Add(string scheme, string colourname, Color colour)
        {
            if (!schemes.ContainsKey(scheme)) return;
            schemes[scheme].AddColour(colourname, colour);
        }

        public static SchemeColour GetColour(string scheme, string colourname)
        {
            if (colourname == "black") return black;
            if (colourname == "white") return white;
            if (!schemes.ContainsKey(scheme)) return null;
            return schemes[scheme].GetColour(colourname);
        }

        public static SchemeColour GetColour(string colourname)
        {
            if (colourname == "black") return black;
            if (colourname == "white") return white;
            if (!schemes.ContainsKey(currentScheme)) return null;
            return schemes[currentScheme].GetColour(colourname);
        }

        public static void Set(string scheme)
        {
            currentScheme = scheme;
        }
    }
    
    public struct FontSetting
    {
        public float size;
        public FontStyle style;
    }

    public static class Fonts
    {
        private static FontFamily family;
        private static Dictionary<string, FontSetting> settings;
        private static Dictionary<string, Font> fonts;
        private static List<string> keys;

        static Fonts()
        {
            family = FontFamily.GenericMonospace;
            settings = new Dictionary<string, FontSetting>();
            fonts = new Dictionary<string, Font>();
            keys = new List<string>();
        }

        public static void Add(string name, float size, FontStyle style = FontStyle.Regular)
        {
            if (settings.ContainsKey(name)) return;
            FontSetting setting = new FontSetting();
            setting.size = size;
            setting.style = style;
            settings.Add(name, setting);
            fonts.Add(name, null);
            keys.Add(name);
        }

        public static Font Get(string name)
        {
            if (!fonts.ContainsKey(name)) return null;
            return fonts[name];
        }

        public static void Recalculate()
        {
            for(int i = 0; i < keys.Count; i++)
            {
                string key = keys[i];
                float size = Drawing.screenSize.Width * settings[key].size;
                Font f = new Font(family, size, settings[key].style, GraphicsUnit.Pixel);
                fonts[key] = f;
            }
        }
    }

    public static class Drawing
    {
        public static Size screenSize;
        public static float ratio;
        public static bool needRedraw = false;

        static Drawing()
        {
            screenSize = new Size(0, 0);
            ratio = 0f;
        }
        
        public static void SetScreen(Size s)
        {
            screenSize = s;
            ratio = (float)s.Width / s.Height;
        }

        public static void DrawRectangle(Graphics g, Rectangle r, string schemeColour)
        {
            SchemeColour colour = ColourSchemes.GetColour(schemeColour);
            if (colour == null) return;
            g.FillRectangle(colour.brush, r);
        }
    }
}