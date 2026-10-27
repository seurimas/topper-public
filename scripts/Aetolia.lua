--[[
  Aetolia.lua -- Aetolia-specific pieces of the Topper backend bridge

  Core.lua is a game-agnostic bridge to the compiled Topper backend process.
  This file supplies everything that is specific to the Aetolia build of
  that backend (topper-aetolia-mudlet.exe):

    * the concrete TopperRequest wrappers it understands (Target/Attack/
      Plan/Hint/Assume/Reset/Api/OpenWeb/BattleStats, and the "db"/"basher"/
      "defup" ModuleMsg shortcuts)
    * Aetolia's GMCP schema (which gmcp.* paths exist and how they're
      collected into a TimeSlice)
    * how to turn Aetolia's GMCP Char.Vitals into the "Stats" prompt variant

  Load this file *after* Core.lua -- it depends on send_request,
  request_module, write_topper, tagged, json_bool, and registerAnonymousEventHandler
  all being defined already, and it supplies build_prompt_stats, which
  Core.lua's send_time_slice() calls but does not itself define.
]]

-- ============================================================
-- Aetolia-specific TopperRequest wrappers
-- ============================================================

function topper_target(who)
  send_request("Target", who)
end

function request_attack(strategy)
  send_request("Attack", strategy)
end

function request_plan(strategy)
  topper_plan = strategy
  send_request("Plan", strategy)
end

function request_api(who)
  send_request("Api", who)
end

function request_web()
  send_request("OpenWeb")
end

function request_hint(who, hint, value)
  send_request("Hint", { who, hint, value })
end

function assume_aff(who, what, value)
  send_request("Assume", { who, what, value == "true" })
end

function request_reset(type)
  write_topper(tagged("Request", tagged("Reset", type)))
end

function request_battle_stats()
  send_request("BattleStats", math.floor(os.clock() * 100))
end

function request_inspect(tree, key)
  request_module("db", "inspect " .. tree .. " " .. key)
end

function request_priority(stack, index, plan)
  request_module("db", "priority " .. stack .. " " .. index .. " " .. plan)
end

function request_hypnosis(stack, index, plan)
  request_module("db", "hypnosis " .. stack .. " " .. index .. " " .. plan)
end

function request_db_hint(key, value)
  request_module("db", "hint " .. key .. " " .. value)
end

function request_db_myhint(key, value)
  request_module("db", "myhint " .. key .. " " .. value)
end

function request_basher(command)
  request_module("basher", command)
end

function request_defup(command)
  request_module("defup", JSON:encode(command))
end

function request_defup_check()
  request_defup("Check")
end

function request_defup_add_command(command, defense)
  request_defup(tagged("AddCommand", { command, defense }))
end

function request_defup_add_trigger(trigger, defense)
  request_defup(tagged("AddUpTrigger", { trigger, defense }))
end

function request_defup_add_down_trigger(trigger, defense)
  request_defup(tagged("AddDownTrigger", { trigger, defense }))
end

function request_defup_remove_command(defense)
  request_defup(tagged("RemoveCommand", { defense }))
end

function request_defup_remove_trigger(trigger)
  request_defup(tagged("RemoveUpTrigger", { trigger }))
end

function request_defup_remove_down_trigger(trigger)
  request_defup(tagged("RemoveDownTrigger", { trigger }))
end

function request_defup_set_player_order(order)
  request_defup(tagged("SetPlayerOrder", { order }))
end

function request_defup_set_class_order(order)
  request_defup(tagged("SetClassOrder", { order }))
end

-- ============================================================
-- Aetolia GMCP schema
-- ============================================================

function handle_gmcp(name)
  if topper_gmcp == nil then
    topper_gmcp = {}
  end
  local gmcp_obj = nil
  if name == "gmcp.Room.Info" then
    gmcp_obj = gmcp.Room.Info
  elseif name == "gmcp.Char.Vitals" then
    gmcp_obj = gmcp.Char.Vitals
  elseif name == "gmcp.Char.Status" then
    gmcp_obj = gmcp.Char.Status
  elseif name == "gmcp.Char.Items.List" then
    gmcp_obj = gmcp.Char.Items.List
  elseif name == "gmcp.Char.Items.Add" then
    gmcp_obj = gmcp.Char.Items.Add
  elseif name == "gmcp.Char.Items.Remove" then
    gmcp_obj = gmcp.Char.Items.Remove
  elseif name == "gmcp.Room.Players" then
    gmcp_obj = gmcp.Room.Players
  elseif name == "gmcp.Room.AddPlayer" then
    gmcp_obj = gmcp.Room.AddPlayer
  elseif name == "gmcp.Room.RemovePlayer" then
    gmcp_obj = gmcp.Room.RemovePlayer
  end
  topper_gmcp[#topper_gmcp + 1] = { name, gmcp_obj }
end

local AETOLIA_GMCP_EVENTS = {
  "gmcp.Room.Info",
  "gmcp.Room.Players",
  "gmcp.Room.AddPlayer",
  "gmcp.Room.RemovePlayer",
  "gmcp.Char.Vitals",
  "gmcp.Char.Items.Add",
  "gmcp.Char.Items.Remove",
  "gmcp.Char.Items.List",
  "gmcp.Char.Status",
}

local function install_aetolia_gmcp_handlers()
  -- registerAnonymousEventHandler is a no-op if the same (event, function)
  -- pair is already registered, so this is safe to call again on reload.
  for _, event in ipairs(AETOLIA_GMCP_EVENTS) do
    registerAnonymousEventHandler(event, "handle_gmcp")
  end
end

install_aetolia_gmcp_handlers()

-- ============================================================
-- Aetolia prompt stats
-- ============================================================

-- Builds the "Stats" prompt variant from Aetolia's GMCP Char.Vitals fields.
function build_aetolia_prompt_stats(vitals)
  local prompt_stats = {}
  prompt_stats.health = tonumber(vitals.hp)
  prompt_stats.mana = tonumber(vitals.mp)
  if vitals.psi ~= nil then
    prompt_stats.sp = tonumber(vitals.psi)
  else
    prompt_stats.sp = 0
  end
  prompt_stats.equilibrium = json_bool(vitals.equilibrium)
  prompt_stats.balance = json_bool(vitals.balance)
  prompt_stats.shadow = json_bool(vitals.ability_bal)
  prompt_stats.prone = json_bool(vitals.prone)
  return prompt_stats
end

-- Core.lua's send_time_slice() calls the game-agnostic build_prompt_stats();
-- point it at Aetolia's implementation.
build_prompt_stats = build_aetolia_prompt_stats

start_topper("aetolia")
