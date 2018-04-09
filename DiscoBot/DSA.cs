﻿namespace DiscoBot
{
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;

    using DiscoBot.Auxiliary;
    using DiscoBot.Characters;

    using Discord.Commands;

    public static class Dsa
    {
        public static ICommandContext GeneralContext { get; set; }

        public static Dictionary<string, string> Relation { get; set; } = new Dictionary<string, string>(); // dictionary to match the char

        public static List<ICharacter> Chars { get; set; } = new List<ICharacter>();  // list of all characters

        public static List<Talent> Talente { get; set; } = new List<Talent>();
        
        public static void Startup()
        {
            Relation.Add("The Doctor", "Numeri Illuminus"); // Relation
            Relation.Add("Tardis", "Morla"); // "Numeri Illuminus");
            Relation.Add("DSA Bot", "Morla"); // "Felis Exodus Schattenwald");
            Relation.Add("Morla", "Morla");
            Relation.Add("Rhoktar", "Rhoktar4");

            // relation.Add("Papo","Gwendelson");
            Relation.Add("Papo", "Pump aus der Gosse");
            Relation.Add("Potus", "Potus");

            // relation.Add("Papo", "Pump aus der Gosse");
            foreach (var filename in Directory.GetFiles("helden", "*.xml"))
            {
                Chars.Add(new Character(filename));
                (Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Talente.Add(v));
            }

            Talente = Talente.OrderBy(x => x.Name).ToList();
        }
    }
}