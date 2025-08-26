#!/usr/bin/env python

import json
from collections import namedtuple
from urllib import request

data = json.load(request.urlopen("https://raw.githubusercontent.com/BitCraftToolBox/BitCraft_GameData" +
                                 "/refs/heads/main" +
                                 "/server/region/resource_desc.json"))

Type = namedtuple('Type', ('tier', 'id', 'name', 'disabled'))

BLOCKED_TAGS = ('Bones', 'Depleted Resource', 'Door', 'Energy Font', 'Fruit', 'Insects', 'Note', 'Obstacle',)
BLOCKED_IDS = (763946195,)

data = [Type(
    t['tier'] if t['tier'] > 0 else 0,
    t['id'],
    t['name'].strip(),
    'Interior' in t['name'] or t['tag'] in BLOCKED_TAGS or t['id'] in BLOCKED_IDS
) for t in data]
data = sorted(data, key=lambda t: (t.tier * -1, t.id))

n = sum((not t.disabled for t in data))

with open('src/resource.rs', 'w') as f:
    f.write('pub struct Resource { pub tier: u8, pub id: i32 }\n' +
            f'\npub const RESOURCES: [Resource; {n}] = [\n')

    for t in data:
        f.write('//' if t.disabled else '  ')
        f.write(f'  Resource {{ tier: {t.tier:>2}, id: {t.id:>11} }}, // {t.name}\n')

    f.write('];\n')
