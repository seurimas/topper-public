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

local gmcp_flush_pending = false

-- Sends GMCP that arrived without any game lines to topper as a promptless, lineless TimeSlice.
local function flush_idle_gmcp()
  gmcp_flush_pending = false
  if topper_now == nil or topper_gmcp == nil or #topper_gmcp == 0 then
    return
  end
  if topper_lines ~= nil and #topper_lines > 0 then
    return
  end
  write_topper(tagged("TimeSlice", build_time_slice("Promptless")))
  observations = {}
  topper_lines = {}
  topper_gmcp = {}
end

function handle_gmcp(name)
  if topper_gmcp == nil then
    topper_gmcp = {}
  end
  topper_gmcp[#topper_gmcp + 1] = { name, gmcp_payload(name) }
  if not gmcp_flush_pending then
    gmcp_flush_pending = true
    tempTimer(0.1, flush_idle_gmcp)
  end
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

-- The backend sends each loaded zone as a table in the map.zone_data data entry.
topper_data_handlers = topper_data_handlers or {}
topper_data_handlers["map.zone_data"] = function(value)
  if duris_load_zone == nil then
    cecho("<red>Map: Duris_Mapper.lua is not loaded; zone data dropped\n")
    return
  end
  duris_load_zone(value)
end

-- The backend sends the current room's mobs as a table in the mobs.room data entry.
topper_data_handlers["mobs.room"] = function(value)
  if duris_show_mobs == nil then
    return
  end
  duris_show_mobs(value)
end

-- The backend sends worn equipment as a table in the equipment data entry.
topper_data_handlers["equipment"] = function(value)
  if duris_show_equipment == nil then
    return
  end
  duris_show_equipment(value)
end

-- The backend sends shop listing history line numbers mapped to item descriptions in the annotate data entry.
topper_data_handlers["annotate"] = function(value)
  if duris_annotate == nil then
    return
  end
  duris_annotate(value)
end

-- The backend sends the overland map (vnum 0) as colored cells in the map.wilderness data entry.
topper_data_handlers["map.wilderness"] = function(value)
  if duris_show_wilderness == nil then
    return
  end
  duris_show_wilderness(value)
end

-- The backend sends the autobash state as a table in the autobash data entry.
topper_data_handlers["autobash"] = function(value)
  if duris_show_autobash == nil then
    return
  end
  duris_show_autobash(value)
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

-- ============================================================
-- Autobash
-- ============================================================

-- Usage: autobash <min level> <max level> <alignments...>, toggles hunting on and off.
-- With fight_only, attacks the mob in the current room and goes inactive when the fight ends.
function duris_autobash(low, high, alignment_text, fight_only)
  local alignments = {}
  for alignment in string.gmatch(alignment_text, "%S+") do
    alignments[#alignments + 1] = alignment
  end
  request_module("autobash", JSON:encode({
    type = "start",
    between = { tonumber(low), tonumber(high) },
    alignments = alignments,
    fight_only = fight_only == true,
  }))
end

function duris_autobash_stop()
  request_module("autobash", JSON:encode({ type = "stop" }))
end

register_topper_command("Autobash", "^autobash (\\d+) (\\d+) (.+)$", "duris_autobash(matches[2], matches[3], matches[4])", "autobash <lo> <hi> <align...>", "Toggle auto-hunting mobs in a level range and alignments (good, evil, neutral)")
register_topper_command("AutobashFight", "^autobash fight (\\d+) (\\d+) (.+)$", "duris_autobash(matches[2], matches[3], matches[4], true)", "autobash fight <lo> <hi> <align...>", "Attack the mob in this room and stop when the fight is over")
register_topper_command("AutobashStop", "^autobash (off|stop)$", "duris_autobash_stop()", "autobash stop", "Stop auto-hunting")

-- ============================================================
-- Helper functions for the Duris module
-- ============================================================

function do_go(direction)
  if gmcp.Room.Info.doors and gmcp.Room.Info.doors[direction] and gmcp.Room.Info.doors[direction].closed then
    send('open ' .. direction)
  end
  send(direction)
end

start_topper("duris")
