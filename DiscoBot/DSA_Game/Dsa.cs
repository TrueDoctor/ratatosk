﻿namespace DiscoBot.DSA_Game
{
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;

    using DiscoBot.Audio;
    using DiscoBot.Auxiliary;
    using DiscoBot.Commands;
    using DiscoBot.DSA_Game.Characters;
    using DiscoBot.DSA_Game.Save;

    using Discord.Commands;

    public static class Dsa
    {
        public static ICommandContext GeneralContext { get; set; }

        public static AudioService Service { get; set; }

        public static Dictionary<string, string> Relation { get; set; } = new Dictionary<string, string>(); // dictionary to match the char

        public static List<ICharacter> Chars { get; set; } = new List<ICharacter>();  // list of all characters

        public static List<Talent> Talente { get; set; } = new List<Talent>();

        public static Properties Properties { get; set; }
        
        public static void Startup()
        {
            Relation.Add("The Doctor", "Numeri Illuminus"); // Relation
            Relation.Add("Tardis", "Helga von Drachenei, Tausendsasserin"); // "Numeri Illuminus");
            Relation.Add("DSA Bot", "Morla"); // "Felis Exodus Schattenwald");
            Relation.Add("Morla", "Morla");
            Relation.Add("Rhoktar", "Rhoktar4");
            Relation.Add("MagicBro5", "Krenko");
            Relation.Add("Nicolas", "Hartmut Reiher");
            Relation.Add("TrueKuehli", "Ledur Torfinson");

             //Relation.Add("Papo","Gwendelson");
            //Relation.Add("Papo", "Pump aus der Gosse");

            //Nachteile für LE, AE, MR
            // Relation.Add("Papo", "Angilbert Arres");

            //Vorteile für LE, AE, MR
            Relation.Add("Papo", "Beef");
            //Relation.Add("Papo", "Astrallos");

            Relation.Add("Potus", "Potus");
            
            // relation.Add("Papo", "Pump aus der Gosse");
            foreach (var filename in Directory.GetFiles("helden", "*.xml"))
            {
                Chars.Add(new Character(filename));
                (Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Talente.Add(v));
            }

            Properties = Save.Properties.Deserialize();
            Properties.Serialize();

            Talente = Talente.OrderBy(x => x.Name).ToList();
        }
    }
}