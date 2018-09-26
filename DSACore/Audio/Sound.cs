﻿namespace DSACore.Audio
{
    public class Sound
    {
        public Sound(string name, string url, int volume)
        {
            this.Name = name;
            this.Url = url;
            this.Volume = volume;
        }

        public string Name { get; }

        public string Url { get; }

        public int Volume { get; }
    }
}