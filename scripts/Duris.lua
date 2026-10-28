--[[
  Duris.lua -- Duris-specific pieces of the Topper backend bridge

  Core.lua is a game-agnostic bridge to the compiled Topper backend process.
  This file supplies the Duris GMCP schema for that backend:

    * which gmcp.* paths the Duris timeline and map consume
    * how they're collected into topper_gmcp for the TimeSlice

  Load this file *after* Core.lua -- it depends on registerAnonymousEventHandler
  being defined already. Core.lua treats build_prompt_stats as optional, so no
  Stats prompt variant is supplied here.
]]

-- ============================================================
-- Duris GMCP schema
-- ============================================================

-- Duris has no Char.Name; the player's name is the "name" field of Char.Vitals.
GMCP_VITALS_PATH = "gmcp.Char.Vitals"
GMCP_NAME_PATH = "gmcp.Char.Vitals"

-- Every event the backend's Duris timeline (gmcp.rs) and map (maps/gmcp.rs) consume.
local DURIS_GMCP_EVENTS = {
  "gmcp.Char.Vitals",
  "gmcp.Char.Status",
  "gmcp.Char.Affects",
  "gmcp.Room.Info",
  "gmcp.Room.Map",
  "gmcp.Combat.Update",
  "gmcp.Group.Status",
  "gmcp.Quest.Status",
  "gmcp.Ship.Info",
  "gmcp.Ship.Contacts",
}

function handle_gmcp(name)
  if topper_gmcp == nil then
    topper_gmcp = {}
  end
  topper_gmcp[#topper_gmcp + 1] = { name, gmcp_payload(name) }
end

local function install_duris_gmcp_handlers()
  -- registerAnonymousEventHandler is a no-op if the same (event, function)
  -- pair is already registered, so this is safe to call again on reload.
  for _, event in ipairs(DURIS_GMCP_EVENTS) do
    registerAnonymousEventHandler(event, "handle_gmcp")
  end
end

install_duris_gmcp_handlers()

-- Sent in one TimeSlice when topper starts (see send_gmcp_snapshot in Core.lua).
topper_snapshot_gmcp_events = {
  "gmcp.Char.Vitals",
  "gmcp.Char.Status",
  "gmcp.Char.Affects",
  "gmcp.Room.Info",
  "gmcp.Group.Status",
  "gmcp.Quest.Status",
  "gmcp.Ship.Info",
  "gmcp.Ship.Contacts",
}

-- ============================================================
-- Map zone load
-- ============================================================

-- The backend sends each loaded zone as a JSON string in the map.zone_data passive.
topper_passive_handlers = topper_passive_handlers or {}
topper_passive_handlers["map.zone_data"] = function(value)
  if duris_load_zone == nil then
    cecho("<red>Map: Duris_Mapper.lua is not loaded; zone data dropped\n")
    return
  end
  duris_load_zone(value)
end

-- The backend sends the current room's mobs as JSON in the mobs.room passive.
topper_passive_handlers["mobs.room"] = function(value)
  if duris_show_mobs == nil then
    return
  end
  duris_show_mobs(value)
end

-- ============================================================
-- Zone reload command
-- ============================================================

function duris_treload()
  request_module("map", JSON:encode({ cmd = "reload_zone" }))
end

register_topper_command("Treload", "^treload$", "duris_treload()", "treload", "Reload the current zone's map data")

function duris_tmap()
  request_module("map", JSON:encode({ cmd = "send_zone" }))
end

register_topper_command("Tmap", "^tmap$", "duris_tmap()", "tmap", "Send the current zone's cached map data")

start_topper("duris")
