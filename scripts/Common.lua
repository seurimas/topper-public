-- Qt key codes and modifier flags, as numbers, since the mudlet.key tables do not resolve.
local SHIFT_KEYPAD = 0x02000000 + 0x20000000

local bindings = {
  { "Home",       0x01000010, "nw"   },
  { "Up",         0x01000013, "n"    },
  { "PgUp",       0x01000016, "ne"   },
  { "Left",       0x01000012, "w"    },
  { "Center",     0x0100000b, "look" },
  { "Right",      0x01000014, "e"    },
  { "End",        0x01000011, "sw"   },
  { "Down",       0x01000015, "s"    },
  { "PgDn",       0x01000017, "se"   },
  { "Out",        0x2f,       "out"  },
  { "In",         0x2a,       "in"   },
  { "Up Level",   0x2d,       "up"   },
  { "Down Level", 0x2b,       "down" },
}

-- Temp key IDs from the previous load, removed so reloading does not stack bindings.
common_keypad_ids = common_keypad_ids or {}
for _, id in pairs(common_keypad_ids) do
  pcall(killKey, id)
end
common_keypad_ids = {}

if do_go == nil then
  function do_go(direction)
    send(direction)
  end
end

for _, binding in ipairs(bindings) do
  local name, key, direction = binding[1], binding[2], binding[3]
  common_keypad_ids[name] = tempKey(SHIFT_KEYPAD, key, function() do_go(direction) end)
end
