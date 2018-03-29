using System;
using System.Collections.Generic;
using System.IO;

namespace Planner
{
    public abstract class DataFile<T>
    {
        protected List<T> list;
        protected string fileName;
        private bool loaded;

        public DataFile(string fileName)
        {
            this.fileName = fileName;
            list = new List<T>();
            loaded = false;
        }

        protected void SetLoaded()
        {
            loaded = true;
        }

        protected bool IsLoaded()
        {
            return loaded;
        }

        public int Size()
        {
            if (!loaded) Load();
            return list.Count;
        }

        public T Get(int i)
        {
            if (!loaded) Load();
            if (i < 0 || i > list.Count - 1) return default(T);
            return list[i];
        }

        public void Add(T i)
        {
            if (!loaded) Load();
            list.Add(i);
        }

        public void Delete(T i)
        {
            if (!loaded) Load();
            list.Remove(i);
        }

        public void Edit(int i, T n)
        {
            if (!loaded) Load();
            list[i] = n;
        }

        public abstract void Load();
        public abstract void Write();
    }

    public class DeadlineFile : DataFile<Deadline>
    {
        public DeadlineFile(string fileName) : base(fileName) { }

        public override void Load()
        {
            if (!File.Exists(fileName)) return;
            list.Clear();
            BinaryReader r = new BinaryReader(File.Open(fileName, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Deadline d = new Deadline();
                d.deadline = Schedule.ReadDateTime(r);
                d.title = r.ReadString();
                d.category = r.ReadString();
                list.Add(d);
            }
            r.Close();
            SetLoaded();
        }

        public override void Write()
        {
            BinaryWriter w = new BinaryWriter(File.Open(fileName, FileMode.OpenOrCreate));
            w.Write(list.Count);
            for (int i = 0; i < list.Count; i++)
            {
                Schedule.WriteDateTime(w, list[i].deadline);
                w.Write(list[i].title);
                w.Write(list[i].category);
            }
            w.Close();     
        }

        public bool Get(DateTime origDt, bool onlyDate, out Deadline result, out int index)
        {
            result = default(Deadline);
            index = 0;
            if (!IsLoaded()) Load();
            for (int i = 0; i < list.Count; i++)
            {
                bool same = Schedule.SameDateTime(origDt, onlyDate, list[i].deadline);
                if (same)
                {
                    index = i;
                    result = list[i];
                    return true;
                }
            }
            return false;
        }
    }

    public class CardFile : DataFile<Card>
    {
        public CardFile(string fileName) : base(fileName) { }

        public override void Load()
        {
            if (!File.Exists(fileName)) return;
            list.Clear();
            BinaryReader r = new BinaryReader(File.Open(fileName, FileMode.Open));
            int count = r.ReadInt32();
            for (int i = 0; i < count; i++)
            {
                Card c = new Card();
                c.start = Schedule.ReadDateTime(r);
                c.end = Schedule.ReadDateTime(r);
                c.title = r.ReadString();
                c.content = r.ReadString();
                c.category = r.ReadString();
                list.Add(c);
            }
            r.Close();
            SetLoaded();
        }

        public override void Write()
        {
            BinaryWriter w = new BinaryWriter(File.Open(fileName, FileMode.OpenOrCreate));
            w.Write(list.Count);
            for (int i = 0; i < list.Count; i++)
            {
                Schedule.WriteDateTime(w, list[i].start);
                Schedule.WriteDateTime(w, list[i].end);
                w.Write(list[i].title);
                w.Write(list[i].content);
                w.Write(list[i].category);
            }
            w.Close();
        }

        public bool Get(DateTime origDt, bool onlyDate, out Card result, out int index)
        {
            result = default(Card);
            index = 0;
            if (!IsLoaded()) Load();
            for (int i = 0; i < list.Count; i++)
            {
                bool same = Schedule.SameDateTime(origDt, onlyDate, list[i].start);
                if (same)
                {
                    index = i;
                    result = list[i];
                    return true;
                }
            }
            return false;
        }
    }
}