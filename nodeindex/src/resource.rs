pub struct Resource { pub tier: u8, pub id: i32 }

pub const RESOURCES: [Resource; 314] = [
    Resource { tier: 10, id:         102 }, // White Lily
    Resource { tier: 10, id:    70663203 }, // Tier 10 Boulder
    Resource { tier: 10, id:    93152192 }, // Flawless Hieroglyphs
//  Resource { tier: 10, id:   370078223 }, // Astralite Outcrop Interior
    Resource { tier: 10, id:   374159821 }, // School Of Flawless Lakefish
    Resource { tier: 10, id:   424796674 }, // School Of Flawless Ocean Fish
    Resource { tier: 10, id:   473828668 }, // Bamboo
    Resource { tier: 10, id:   939382648 }, // Wisteria Tree
    Resource { tier: 10, id:  1006230316 }, // Frenzied School Of Flawless Ocean Fish
    Resource { tier: 10, id:  1023127595 }, // Flawless Clay
    Resource { tier: 10, id:  1159270109 }, // Tier 10 Tree
    Resource { tier: 10, id:  1283711960 }, // Flawless Baitfish
    Resource { tier: 10, id:  1333270269 }, // Flawless Sand
//  Resource { tier: 10, id:  1357154092 }, // Astralite Vein Interior Depleted
//  Resource { tier: 10, id:  1444297495 }, // Medium Astralite Vein Interior Depleted
//  Resource { tier: 10, id:  1452484871 }, // Medium Astralite Vein Interior
    Resource { tier: 10, id:  1467799531 }, // Dewberry Bush
    Resource { tier: 10, id:  1489491467 }, // Tier 10 Fibers
//  Resource { tier: 10, id:  1526350171 }, // Astralite Outcrop Interior Depleted
    Resource { tier: 10, id:  1606770008 }, // Astralite Vein
    Resource { tier: 10, id:  1637125903 }, // Flawless Mushroom
//  Resource { tier: 10, id:  1747556974 }, // Astralite Vein Interior
    Resource { tier: 10, id:  1986100626 }, // Tier 10 Flower
    Resource { tier: 10, id:  1996631377 }, // Tier 10 Outcrop
    Resource { tier: 10, id:  2089197796 }, // Enoki
    Resource { tier: 10, id:  2110330714 }, // Flawless Berry Bush
    Resource { tier:  9, id:          83 }, // King Trumpet
    Resource { tier:  9, id:          86 }, // Citriformis
//  Resource { tier:  9, id:    20400246 }, // Umbracite Vein Interior Depleted
//  Resource { tier:  9, id:    64125498 }, // Umbracite Vein Interior
    Resource { tier:  9, id:   189403270 }, // Tier 9 Fibers
    Resource { tier:  9, id:   284200468 }, // Tier 9 Flower
    Resource { tier:  9, id:   331687458 }, // Magnificient Hieroglyphs
    Resource { tier:  9, id:   562432497 }, // Magnificient Sand
//  Resource { tier:  9, id:   749656892 }, // Medium Umbracite Vein Interior Depleted
    Resource { tier:  9, id:   756579517 }, // Magnificient Baitfish
//  Resource { tier:  9, id:   814703516 }, // Umbracite Outcrop Interior
    Resource { tier:  9, id:   939701809 }, // Gigantic Sapwood Tree
    Resource { tier:  9, id:   963451338 }, // Magnificient Berry Bush
    Resource { tier:  9, id:  1113640469 }, // Tier 9 Outcrop
    Resource { tier:  9, id:  1141184831 }, // Frenzied School Of Magnificient Ocean Fish
    Resource { tier:  9, id:  1157887989 }, // School Of Magnificient Lake Fish
//  Resource { tier:  9, id:  1241355606 }, // Umbracite Outcrop Interior Depleted
//  Resource { tier:  9, id:  1384946093 }, // Medium Umbracite Vein Interior
    Resource { tier:  9, id:  1386735112 }, // Umbracite Vein
    Resource { tier:  9, id:  1458811602 }, // Seaweed
    Resource { tier:  9, id:  1526038154 }, // Magnificient Clay
    Resource { tier:  9, id:  1574437474 }, // Sapwood Tree
    Resource { tier:  9, id:  1742959882 }, // Magnificient Mushroom
    Resource { tier:  9, id:  1812221896 }, // School Of Magnificient Ocean Fish
    Resource { tier:  9, id:  1821415333 }, // Mature Sapwood Tree
    Resource { tier:  9, id:  1902966974 }, // Tier 9 Boulder
    Resource { tier:  9, id:  1954847232 }, // Black Fig Bush
    Resource { tier:  8, id:          84 }, // Cloudberry Bush
    Resource { tier:  8, id:          85 }, // Ghost Mushroom
    Resource { tier:  8, id:         105 }, // King of the Alps
    Resource { tier:  8, id:         137 }, // Arctic Grass
    Resource { tier:  8, id:     2390533 }, // School Of Mysterious Anglerfish
//  Resource { tier:  8, id:    20857160 }, // Celestium Outcrop Interior
//  Resource { tier:  8, id:    71559749 }, // Celestium Vein Interior Depleted
    Resource { tier:  8, id:   457752715 }, // Pristine Sand
    Resource { tier:  8, id:   479638263 }, // Tier 8 Boulder
    Resource { tier:  8, id:   505488132 }, // Pristine Clay
    Resource { tier:  8, id:   509854054 }, // Frenzied School Of Mysterious Anglerfish
//  Resource { tier:  8, id:   650019671 }, // Medium Celestium Vein Interior Depleted
    Resource { tier:  8, id:   722506673 }, // School Of Rainbowscaled Tilapia
    Resource { tier:  8, id:  1101060328 }, // Ficus Tree
    Resource { tier:  8, id:  1125409070 }, // Tier 8 Fibers
//  Resource { tier:  8, id:  1162923141 }, // Celestium Vein Interior
    Resource { tier:  8, id:  1264935363 }, // Tier 8 Flower
    Resource { tier:  8, id:  1332797261 }, // Celestium Vein
    Resource { tier:  8, id:  1423928615 }, // Tier 8 Outcrop
    Resource { tier:  8, id:  1558728865 }, // Sparkling Prawn
    Resource { tier:  8, id:  1567694896 }, // Pristine Hieroglyphs
    Resource { tier:  8, id:  1592739620 }, // Pristine Berry Bush
    Resource { tier:  8, id:  1657885116 }, // Pristine Mushroom
//  Resource { tier:  8, id:  1731709368 }, // Overwhelming Hexite Energy Font
//  Resource { tier:  8, id:  2025189123 }, // Medium Celestium Vein Interior
//  Resource { tier:  8, id:  2140754992 }, // Celestium Outcrop Interior Depleted
    Resource { tier:  7, id:          35 }, // Young Baobab Tree
    Resource { tier:  7, id:          36 }, // Ancient Baobab Tree
    Resource { tier:  7, id:          76 }, // Oyster Mushrooms
    Resource { tier:  7, id:         104 }, // Golden Witlow
    Resource { tier:  7, id:         128 }, // Grassy Reeds
    Resource { tier:  7, id:         129 }, // Reindeer Lichen
    Resource { tier:  7, id:     5045122 }, // School Of Tidebreaker Barracuda
    Resource { tier:  7, id:   139483458 }, // Aurumite Vein
    Resource { tier:  7, id:   387666932 }, // Tier 7 Tree
//  Resource { tier:  7, id:   411376268 }, // Aurumite Vein Interior Depleted
    Resource { tier:  7, id:   582591086 }, // Ornate Berry Bush
    Resource { tier:  7, id:   586543849 }, // Ornate Mushroom
    Resource { tier:  7, id:   723013812 }, // Tier 7 Flower
    Resource { tier:  7, id:   826362353 }, // Frenzied School Of Tidebreaker Barracuda
    Resource { tier:  7, id:   834195042 }, // Ornate Clay
    Resource { tier:  7, id:   904022325 }, // School Of Speedy Glowfin
//  Resource { tier:  7, id:  1171246287 }, // Medium Aurumite Vein Interior Depleted
    Resource { tier:  7, id:  1262898141 }, // Misty Crustacean
    Resource { tier:  7, id:  1332535555 }, // Ornate Sand
    Resource { tier:  7, id:  1440062914 }, // Tier 7 Boulder
    Resource { tier:  7, id:  1579330042 }, // Yellow Apricot Bush
//  Resource { tier:  7, id:  1689263994 }, // Aurumite Outcrop Interior Depleted
//  Resource { tier:  7, id:  1800013378 }, // Medium Aurumite Vein Interior
    Resource { tier:  7, id:  1981854097 }, // Tier 7 Fibers
//  Resource { tier:  7, id:  2031243337 }, // Aurumite Vein Interior
//  Resource { tier:  7, id:  2073862342 }, // Aurumite Outcrop Interior
    Resource { tier:  7, id:  2104975743 }, // Tier 7 Outcrop
    Resource { tier:  7, id:  2124845482 }, // Ornate Hieroglyphs
    Resource { tier:  6, id:           5 }, // Maple Sapling
    Resource { tier:  6, id:          17 }, // Young Maple Tree
    Resource { tier:  6, id:          23 }, // Ancient Oak Tree
    Resource { tier:  6, id:          24 }, // Gnarled Maple Tree
    Resource { tier:  6, id:          25 }, // Mature Maple Tree
    Resource { tier:  6, id:          56 }, // Marble Outcrop
    Resource { tier:  6, id:          57 }, // Marble Boulder
    Resource { tier:  6, id:          65 }, // Rathium Vein
    Resource { tier:  6, id:          71 }, // Bentonite Clay
    Resource { tier:  6, id:          81 }, // Morel Mushrooms
    Resource { tier:  6, id:          99 }, // Desert Rose
    Resource { tier:  6, id:         101 }, // Fireweed
    Resource { tier:  6, id:         139 }, // Ancient Thorns
    Resource { tier:  6, id:         145 }, // Peerless Hieroglyphs
//  Resource { tier:  6, id:         201 }, // Energy Source
//  Resource { tier:  6, id:         202 }, // Ornate Key Pedestal
//  Resource { tier:  6, id:         203 }, // Advanced Key Pedestal
//  Resource { tier:  6, id:         204 }, // Power Source
//  Resource { tier:  6, id:         205 }, // Broken Power Source
    Resource { tier:  6, id:         206 }, // Collapsed Marble Pillars
//  Resource { tier:  6, id:         207 }, // Advanced Door
//  Resource { tier:  6, id:         208 }, // Right Power Core Pedestal
//  Resource { tier:  6, id:         209 }, // Empty Power Core Pedestal
//  Resource { tier:  6, id:         210 }, // Runic Door
    Resource { tier:  6, id:         211 }, // Trap Rubble
//  Resource { tier:  6, id:         212 }, // Runic Door
//  Resource { tier:  6, id:         213 }, // Mysterious Contraption
//  Resource { tier:  6, id:         214 }, // Powered Contraption
//  Resource { tier:  6, id:         223 }, // Left Power Core Pedestal
    Resource { tier:  6, id:     3010008 }, // Planted Maple Sapling
    Resource { tier:  6, id:     3011008 }, // Planted Maple Tree
    Resource { tier:  6, id:     3012008 }, // Fully Grown Maple Tree
    Resource { tier:  6, id:     6110000 }, // Sunrise Shrimp
    Resource { tier:  6, id:     6110001 }, // School Of Hexfin Perch
    Resource { tier:  6, id:     6110002 }, // School Of Abyssal Swordfish
    Resource { tier:  6, id:     6110003 }, // Frenzied School Of Abyssal Swordfish
    Resource { tier:  6, id:     6110004 }, // Abyssal Oyster
//  Resource { tier:  6, id:   246146358 }, // Rathium Vein Interior Depleted
    Resource { tier:  6, id:   368570220 }, // Cranberry Bush
    Resource { tier:  6, id:   702104027 }, // Clay Termite Mound
//  Resource { tier:  6, id:   750444302 }, // Medium Rathium Vein Interior Depleted
    Resource { tier:  6, id:   762731569 }, // Palmetto
//  Resource { tier:  6, id:   773149133 }, // Medium Rathium Vein Interior
    Resource { tier:  6, id:   875245395 }, // Mistberry Bush
//  Resource { tier:  6, id:   916586661 }, // Rathium Outcrop Interior
//  Resource { tier:  6, id:   932989637 }, // Powerful Hexite Energy Font
    Resource { tier:  6, id:   999376882 }, // Desert Sand
//  Resource { tier:  6, id:  1318826480 }, // Rathium Outcrop Interior Depleted
//  Resource { tier:  6, id:  2027405944 }, // Rathium Vein Interior
    Resource { tier:  5, id:          30 }, // Stunted Cypress Tree
    Resource { tier:  5, id:          31 }, // Dead Cypress Tree
    Resource { tier:  5, id:          33 }, // Mature Cypress Tree
    Resource { tier:  5, id:          34 }, // Ancient Cypress Tree
    Resource { tier:  5, id:          54 }, // Diorite Outcrop
    Resource { tier:  5, id:          55 }, // Diorite Boulder
    Resource { tier:  5, id:          64 }, // Luminite Vein
    Resource { tier:  5, id:          70 }, // Kaolinite Clay
    Resource { tier:  5, id:          75 }, // Blackberry Bush
    Resource { tier:  5, id:          82 }, // Truffle Patch
    Resource { tier:  5, id:          98 }, // Morning Glory
    Resource { tier:  5, id:         103 }, // Ghost Thyme
    Resource { tier:  5, id:         135 }, // Heather
    Resource { tier:  5, id:         136 }, // Large Brambles
    Resource { tier:  5, id:         138 }, // Pink Lilies
    Resource { tier:  5, id:         144 }, // Exquisite Hieroglyphs
    Resource { tier:  5, id:     4011011 }, // Planted Cypress Tree
    Resource { tier:  5, id:     4012011 }, // Fully Grown Cypress Tree
//  Resource { tier:  5, id:     5050000 }, // Garden Pillar Interior
//  Resource { tier:  5, id:     5050001 }, // Garden Formation Interior
//  Resource { tier:  5, id:     5050002 }, // Diorite Pillar Interior
    Resource { tier:  5, id:     5110000 }, // Golden Crawfish
    Resource { tier:  5, id:     5110001 }, // School Of Emberscale Sturgeon
    Resource { tier:  5, id:     5110002 }, // School Of Azure Sharks
    Resource { tier:  5, id:     5110003 }, // Frenzied School Of Azure Sharks
    Resource { tier:  5, id:     5110004 }, // Armored Reef Clam
//  Resource { tier:  5, id:    49710228 }, // Luminite Outcrop Interior Depleted
//  Resource { tier:  5, id:   549538391 }, // Medium Luminite Vein Interior
//  Resource { tier:  5, id:   642972236 }, // Luminite Outcrop Interior
//  Resource { tier:  5, id:   887736443 }, // Luminite Vein Interior
    Resource { tier:  5, id:  1005142992 }, // Elephant Fibers
//  Resource { tier:  5, id:  1126774401 }, // Medium Luminite Vein Interior Depleted
//  Resource { tier:  5, id:  1365934955 }, // Luminite Vein Interior Depleted
    Resource { tier:  5, id:  1691492474 }, // Coral Sand
    Resource { tier:  4, id:          16 }, // Young Pine Tree
    Resource { tier:  4, id:          26 }, // Mature Pine Tree
    Resource { tier:  4, id:          32 }, // Ancient Pine Tree
    Resource { tier:  4, id:          52 }, // Basalt Outcrop
    Resource { tier:  4, id:          53 }, // Basalt Stalagmite
    Resource { tier:  4, id:          63 }, // Elenvar Vein
    Resource { tier:  4, id:          69 }, // Fine Clay
    Resource { tier:  4, id:          92 }, // Bluebell
    Resource { tier:  4, id:         100 }, // Rosemary
    Resource { tier:  4, id:         133 }, // Thorny Stump
    Resource { tier:  4, id:         134 }, // Brambles
    Resource { tier:  4, id:         143 }, // Fine Hieroglyphs
    Resource { tier:  4, id:     2050000 }, // Garden Boulder
    Resource { tier:  4, id:     3011009 }, // Planted Pine Tree
    Resource { tier:  4, id:     3012009 }, // Fully Grown Pine Tree
    Resource { tier:  4, id:     4050000 }, // Garden Pillar
    Resource { tier:  4, id:     4050001 }, // Garden Formation
    Resource { tier:  4, id:     4050002 }, // Rocky Garden Pillar
    Resource { tier:  4, id:     4050003 }, // Rocky Garden Pillars
    Resource { tier:  4, id:     4050004 }, // Rocky Garden Formation
    Resource { tier:  4, id:     4050005 }, // Diorite Pillar
    Resource { tier:  4, id:     4050006 }, // Large Rocky Garden Pillars
    Resource { tier:  4, id:     4050007 }, // Large Rocky Garden Formations
    Resource { tier:  4, id:     4110000 }, // Pygmy Lobsters
    Resource { tier:  4, id:     4110001 }, // School Of Mossfin Chub
    Resource { tier:  4, id:     4110002 }, // School Of Seastorm Tuna
    Resource { tier:  4, id:     4110003 }, // Frenzied School Of Seastorm Tuna
    Resource { tier:  4, id:     4110004 }, // Crystal Shell Scallop
//  Resource { tier:  4, id:   205387239 }, // Medium Elenvar Vein Interior Depleted
    Resource { tier:  4, id:   532077242 }, // Indigo Milk Cap
    Resource { tier:  4, id:   541862086 }, // Garnet Sand
    Resource { tier:  4, id:   715451185 }, // Blueberry Bush
//  Resource { tier:  4, id:   782933576 }, // Elenvar Outcrop Interior
//  Resource { tier:  4, id:   789563787 }, // Strong Hexite Energy Font
//  Resource { tier:  4, id:   806722041 }, // Elenvar Vein Interior Depleted
//  Resource { tier:  4, id:  1077990023 }, // Elenvar Vein Interior
    Resource { tier:  4, id:  1566846336 }, // Rosewood Tree
//  Resource { tier:  4, id:  1709170104 }, // Elenvar Outcrop Interior Depleted
    Resource { tier:  4, id:  1917261269 }, // Jute
//  Resource { tier:  4, id:  2066552867 }, // Medium Elenvar Vein Interior
    Resource { tier:  3, id:          15 }, // Young Spruce Tree
    Resource { tier:  3, id:          22 }, // Large Stump
    Resource { tier:  3, id:          27 }, // Mature Spruce Tree
    Resource { tier:  3, id:          28 }, // Large Fallen Tree
    Resource { tier:  3, id:          50 }, // Granite Boulder
    Resource { tier:  3, id:          51 }, // Granite Outcrop
    Resource { tier:  3, id:          62 }, // Emarium Vein
    Resource { tier:  3, id:          68 }, // Earthenware Clay
    Resource { tier:  3, id:          78 }, // Juniper Berry Bush
    Resource { tier:  3, id:          93 }, // Aloe
    Resource { tier:  3, id:          95 }, // Snowdrop Flowers
    Resource { tier:  3, id:          96 }, // Marigold
    Resource { tier:  3, id:          97 }, // Thyme
    Resource { tier:  3, id:         130 }, // Ghost Succulent
    Resource { tier:  3, id:         131 }, // Bullrushes
    Resource { tier:  3, id:         132 }, // Spanish Moss
    Resource { tier:  3, id:         142 }, // Neat Hieroglyphs
    Resource { tier:  3, id:         153 }, // Fossils
    Resource { tier:  3, id:         154 }, // Sandcovered Fossils
    Resource { tier:  3, id:         215 }, // Vines T3
    Resource { tier:  3, id:         216 }, // Root T3
    Resource { tier:  3, id:         217 }, // Rubble T3
//  Resource { tier:  3, id:         218 }, // Enadarite Stand
//  Resource { tier:  3, id:         219 }, // Key Mold Pedestal
//  Resource { tier:  3, id:         220 }, // Empty Enadarite Stand
//  Resource { tier:  3, id:         221 }, // Empty Key Mold Pedestal
//  Resource { tier:  3, id:         222 }, // Enadarite Door
    Resource { tier:  3, id:     1000028 }, // Large Fallen Tree Stump
    Resource { tier:  3, id:     1000029 }, // Fallen Grove Tree
    Resource { tier:  3, id:     3011010 }, // Planted Spruce Tree
    Resource { tier:  3, id:     3011011 }, // Planted Willow Tree
    Resource { tier:  3, id:     3012010 }, // Fully Grown Spruce Tree
    Resource { tier:  3, id:     3012011 }, // Fully Grown Willow Tree
    Resource { tier:  3, id:     3020001 }, // Medium Emarium Vein
    Resource { tier:  3, id:     3020002 }, // Large Emarium Vein
    Resource { tier:  3, id:     3110000 }, // Hunchback Prawns
    Resource { tier:  3, id:     3110001 }, // School Of Coralcrest Darter
    Resource { tier:  3, id:     3110002 }, // School Of Wavecrest Eels
    Resource { tier:  3, id:     3110003 }, // Frenzied School Of Wavecrest Eels
    Resource { tier:  3, id:     3110004 }, // Pearlback Snail
//  Resource { tier:  3, id:   198759779 }, // Emarium Outcrop Interior
//  Resource { tier:  3, id:   379219978 }, // Medium Emarium Vein Interior
//  Resource { tier:  3, id:   986344159 }, // Medium Emarium Vein Interior Depleted
//  Resource { tier:  3, id:  1064484466 }, // Emarium Outcrop Interior Depleted
    Resource { tier:  3, id:  1072537375 }, // Russala
    Resource { tier:  3, id:  1180909566 }, // Crystalized Sand
    Resource { tier:  3, id:  1908426535 }, // Windswept Tree
//  Resource { tier:  3, id:  2068688558 }, // Emarium Vein Interior Depleted
//  Resource { tier:  3, id:  2072988001 }, // Emarium Vein Interior
    Resource { tier:  2, id:           9 }, // Birch Sapling
    Resource { tier:  2, id:          13 }, // Young Birch Tree
    Resource { tier:  2, id:          18 }, // Dendro Tree
    Resource { tier:  2, id:          20 }, // Mature Birch Tree
    Resource { tier:  2, id:          29 }, // Large Tree Stump
    Resource { tier:  2, id:          46 }, // Shale Boulder
    Resource { tier:  2, id:          47 }, // Shale Outcrop
    Resource { tier:  2, id:          48 }, // Sandstone Boulder
    Resource { tier:  2, id:          49 }, // Sandstone Outcrop
    Resource { tier:  2, id:          61 }, // Pyrelite Vein
    Resource { tier:  2, id:          67 }, // Clay Mound
    Resource { tier:  2, id:          77 }, // Prickly Pear
    Resource { tier:  2, id:          79 }, // Chanterelles
    Resource { tier:  2, id:          80 }, // Honeyberry Bush
    Resource { tier:  2, id:          87 }, // Dandelions
    Resource { tier:  2, id:          94 }, // Peppermint
    Resource { tier:  2, id:         125 }, // Ferns
    Resource { tier:  2, id:         127 }, // Pine Weed
    Resource { tier:  2, id:         141 }, // Simple Hieroglyphs
    Resource { tier:  2, id:         176 }, // Collapsed Pillars
    Resource { tier:  2, id:         179 }, // Vines T2
    Resource { tier:  2, id:         180 }, // Root T2
    Resource { tier:  2, id:         181 }, // Rubble T2
//  Resource { tier:  2, id:         190 }, // Key Pedestal
//  Resource { tier:  2, id:         191 }, // Empty Key Pedestal
//  Resource { tier:  2, id:         192 }, // Intricate Door
//  Resource { tier:  2, id:         193 }, // Open Door
//  Resource { tier:  2, id:         194 }, // Ancient Adventurer's Note
    Resource { tier:  2, id:     1010008 }, // Planted Birch Sapling
    Resource { tier:  2, id:     1011008 }, // Planted Birch Tree
    Resource { tier:  2, id:     1012008 }, // Fully Grown Birch Tree
    Resource { tier:  2, id:     2011008 }, // Planted Dendro Tree
    Resource { tier:  2, id:     2012008 }, // Fully Grown Dendro Tree
    Resource { tier:  2, id:     2020001 }, // Medium Pyrelite Vein
    Resource { tier:  2, id:     2020002 }, // Large Pyrelite Vein
    Resource { tier:  2, id:     2050001 }, // Large Garden Boulder
    Resource { tier:  2, id:     2110000 }, // Driftwood Crayfish
    Resource { tier:  2, id:     2110001 }, // School Of Emberfin Shiners
    Resource { tier:  2, id:     2110002 }, // School Of Serpentfish
    Resource { tier:  2, id:     2110003 }, // Frenzied School Of Serpentfish
    Resource { tier:  2, id:     2110004 }, // Tough Shelled Mussel
    Resource { tier:  2, id:     2140000 }, // Giant Groundsel Plant
//  Resource { tier:  2, id:   134935169 }, // Pyrelite Vein Interior Depleted
    Resource { tier:  2, id:   464034838 }, // Olivine Sand
//  Resource { tier:  2, id:   699727318 }, // Medium Pyrelite Vein Interior
//  Resource { tier:  2, id:   746946997 }, // Hexite Energy Font
//  Resource { tier:  2, id:  1045808810 }, // Pyrelite Outcrop Interior
//  Resource { tier:  2, id:  1619369727 }, // Pyrelite Vein Interior
//  Resource { tier:  2, id:  1673056013 }, // Terrified Adventurer's Note
//  Resource { tier:  2, id:  1828992183 }, // Pyrelite Outcrop Interior Depleted
//  Resource { tier:  2, id:  1842369948 }, // Medium Pyrelite Vein Interior Depleted
    Resource { tier:  1, id:           1 }, // Sticks
    Resource { tier:  1, id:           2 }, // Bush
    Resource { tier:  1, id:           3 }, // Rotten Log
    Resource { tier:  1, id:           4 }, // Rotten Stump
    Resource { tier:  1, id:           6 }, // Pine Sapling
    Resource { tier:  1, id:           7 }, // Spruce Sapling
    Resource { tier:  1, id:           8 }, // Oak Sapling
    Resource { tier:  1, id:          10 }, // Beech Sapling
    Resource { tier:  1, id:          11 }, // Dead Tree
    Resource { tier:  1, id:          12 }, // Young Beech Tree
    Resource { tier:  1, id:          14 }, // Young Oak Tree
    Resource { tier:  1, id:          19 }, // Mature Beech Tree
    Resource { tier:  1, id:          21 }, // Mature Oak Tree
    Resource { tier:  1, id:          38 }, // Flint Pile
    Resource { tier:  1, id:          40 }, // Limestone Boulder
    Resource { tier:  1, id:          41 }, // Limestone Outcrop
    Resource { tier:  1, id:          42 }, // Large Limestone Outcrop
    Resource { tier:  1, id:          43 }, // Large Limestone Rock
    Resource { tier:  1, id:          44 }, // Large Limestone Boulder
    Resource { tier:  1, id:          45 }, // Limestone Rock Formation
    Resource { tier:  1, id:          58 }, // Ferralith Vein
    Resource { tier:  1, id:          59 }, // Ferralith Outcrop
    Resource { tier:  1, id:          60 }, // Rich Ferralith Vein
    Resource { tier:  1, id:          66 }, // Mud Mound
    Resource { tier:  1, id:          72 }, // Wild Grains
    Resource { tier:  1, id:          73 }, // Strawberry Bush
    Resource { tier:  1, id:          74 }, // Button Mushrooms
    Resource { tier:  1, id:          88 }, // Daisies
    Resource { tier:  1, id:          89 }, // Lavender
    Resource { tier:  1, id:          90 }, // Lavender Cluster
    Resource { tier:  1, id:          91 }, // Small Lavender Cluster
    Resource { tier:  1, id:         140 }, // Rough Hieroglyphs
    Resource { tier:  1, id:         152 }, // Gem Encrusted Stalagmite
    Resource { tier:  1, id:         156 }, // Wild Starbulb Plant
    Resource { tier:  1, id:         158 }, // Salt Deposit
    Resource { tier:  1, id:         159 }, // Vines
    Resource { tier:  1, id:         160 }, // Root
    Resource { tier:  1, id:         161 }, // Rubble
//  Resource { tier:  1, id:         162 }, // Ancient Door
//  Resource { tier:  1, id:         163 }, // Ancient Blue Door
//  Resource { tier:  1, id:         164 }, // Ancient Red Door
//  Resource { tier:  1, id:         165 }, // Ancient Green Door
//  Resource { tier:  1, id:         166 }, // Ancient Yellow Door
//  Resource { tier:  1, id:         167 }, // Complicated Ancient Door
//  Resource { tier:  1, id:         168 }, // Ancient Door
//  Resource { tier:  1, id:         169 }, // Ancient Door
    Resource { tier:  1, id:         170 }, // Large Rubble
//  Resource { tier:  1, id:         171 }, // Ancient Brazier
//  Resource { tier:  1, id:         172 }, // Ancient Brazier
//  Resource { tier:  1, id:         173 }, // Lit Ancient Brazier
//  Resource { tier:  1, id:         174 }, // Broken Ancient Brazier
    Resource { tier:  1, id:         175 }, // Hexite Aurumite Axe
//  Resource { tier:  1, id:         177 }, // Lit Wooden Brazier
//  Resource { tier:  1, id:         178 }, // Wooden Brazier
//  Resource { tier:  1, id:         182 }, // Lit Wooden Brazier
//  Resource { tier:  1, id:         183 }, // Wooden Brazier
    Resource { tier:  1, id:         184 }, // Collapsed Pillar
//  Resource { tier:  1, id:         185 }, // Ancient Adventurer's Letter
//  Resource { tier:  1, id:         186 }, // Ancient Trapsmith's Note
//  Resource { tier:  1, id:         187 }, // Baffled Adventurer's Note
//  Resource { tier:  1, id:         188 }, // Apprehensive Adventurer's Note
//  Resource { tier:  1, id:         189 }, // Helpful Adventurer's Note
//  Resource { tier:  1, id:         195 }, // Powered Door
//  Resource { tier:  1, id:         196 }, // Partially Powered Door
//  Resource { tier:  1, id:         197 }, // Unpowered Door
//  Resource { tier:  1, id:         198 }, // Powered Door Contraption
//  Resource { tier:  1, id:         199 }, // Overloaded Door Contraption
//  Resource { tier:  1, id:         200 }, // Ornate Door
    Resource { tier:  1, id:     1010009 }, // Planted Beech Sapling
    Resource { tier:  1, id:     1010010 }, // Planted Oak Sapling
    Resource { tier:  1, id:     1011009 }, // Planted Beech Tree
    Resource { tier:  1, id:     1011010 }, // Planted Oak Tree
    Resource { tier:  1, id:     1012009 }, // Fully Grown Beech Tree
    Resource { tier:  1, id:     1012010 }, // Fully Grown Oak Tree
    Resource { tier:  1, id:     1020001 }, // Medium Ferralith Vein
    Resource { tier:  1, id:     1090000 }, // Silken Hexmoths
    Resource { tier:  1, id:     1110000 }, // Moonlit Crawdads
    Resource { tier:  1, id:     1110001 }, // School Of Breezy Fin Darters
    Resource { tier:  1, id:     1110002 }, // School Of Oceancrest Marlins
    Resource { tier:  1, id:     1110003 }, // Frenzied School Of Oceancrest Marlins
    Resource { tier:  1, id:     1110004 }, // Seaside Clam
    Resource { tier:  1, id:     2010008 }, // Planted Dendro Sapling
    Resource { tier:  1, id:     3010009 }, // Planted Pine Sapling
    Resource { tier:  1, id:     3010010 }, // Planted Spruce Sapling
    Resource { tier:  1, id:     3010011 }, // Planted Willow Sapling
    Resource { tier:  1, id:     4010011 }, // Planted Cypress Sapling
//  Resource { tier:  1, id:   152428426 }, // Medium Ferralith Vein Interior Depleted
    Resource { tier:  1, id:   182331452 }, // Traveler's Fruit
    Resource { tier:  1, id:   204021372 }, // Rough Sand Pile
//  Resource { tier:  1, id:   218270468 }, // Ferralith Outcrop Interior
//  Resource { tier:  1, id:   322711580 }, // Faint Hexite Energy Font
//  Resource { tier:  1, id:   474230316 }, // Ferralith Outcrop Interior Depleted
    Resource { tier:  1, id:   763946195 }, // TEST BirdSpawn
    Resource { tier:  1, id:  1283632905 }, // Traveler's Fruit (Depleted)
    Resource { tier:  1, id:  1303955933 }, // Traveler's Tree
    Resource { tier:  1, id:  1435874592 }, // Traveler's Tree (Depleted)
//  Resource { tier:  1, id:  1565420196 }, // Medium Ferralith Vein Interior
//  Resource { tier:  1, id:  1917744937 }, // Ferralith Vein Interior Depleted
//  Resource { tier:  1, id:  1962517199 }, // Ferralith Vein Interior
    Resource { tier:  1, id:  2144918116 }, // Daisy
    Resource { tier:  0, id:         500 }, // Depleted Sticks
    Resource { tier:  0, id:         501 }, // Depleted Flint
    Resource { tier:  0, id:  2145270439 }, // Jakyl Den
];
