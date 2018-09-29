﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Group
    {
        private string Name { get; set; }
        private string Discord { get; set; }
        private int Id { get; set; }
        private List<GroupChar> Chars { get; set; }= new List<GroupChar>();
    }
}
