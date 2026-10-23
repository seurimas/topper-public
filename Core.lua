--[[
  Core.lua -- Topper backend bridge for Mudlet (Aetolia)

  This script contains ONLY the components that interact with the compiled
  Rust backend (topper-aetolia-mudlet.exe, see topper-aetolia-mudlet/src/main.rs
  and topper-core-mudlet/src/topper/mod.rs). It is responsible for:

    * launching/monitoring/killing the backend process
    * encoding and sending TimeSlice/Request JSON messages to its stdin
    * decoding and dispatching the JSON responses it writes to stdout
    * collecting the raw inputs (lines, GMCP, sent commands) that make up
      a TimeSlice, via triggers/event handlers this script creates itself

  It intentionally does NOT include the large tree of game-text pattern
  triggers (CombatActions, class "Spoofs", etc.) that used to live alongside
  this code in the exported Mudlet profile. The backend already re-parses
  raw lines itself via ObservationParser::new_from_directory() against the
  JSON trigger definitions in the triggers/ directory of this repository,
  so those Mudlet-side regex triggers are redundant game/content logic, not
  backend plumbing.

  Nothing here depends on any pre-existing GUI-created Trigger/Alias/Script
  from the exported profile; every Mudlet trigger and event handler this
  bridge needs is created by this script when it loads.
]]

-- ============================================================
-- Configuration
-- ============================================================

-- Root of the user's home directory; all backend paths below are relative
-- to this. Adjust if this script is used from a different machine/checkout.
HOME_DIR = HOME_DIR -- Set this, if missing

-- ============================================================
-- Wire format (JSON)
-- ============================================================
-- The backend expects line-delimited JSON. We reuse the same JSON library
-- the original profile used (rather than Mudlet's built-in yajl) because its
-- encoding of empty tables/booleans and the "None" sentinel substitution
-- below are already tuned to match what serde_json expects on the Rust side.
if JSON == nil then
  local jsonPath = HOME_DIR .. [[.config\mudlet\profiles\SyncedAetolia\JSON.lua]]
  local chunk, err = loadfile(jsonPath)
  if chunk then
    JSON = chunk()
  else
    cecho("<red>Core.lua: could not load JSON library from " .. jsonPath .. " (" .. tostring(err) .. ")\n")
  end
end

function json_bool(val)
  if val == "1" then
    return true
  else
    return false
  end
end

-- Wraps a value in a single-key table, e.g. tagged("Stats", {...}) ->
-- { Stats = {...} }. This mirrors the externally-tagged enum representation
-- serde_json uses for TopperRequest/TopperMessage/AetObservation.
function tagged(name, value)
  local t = {}
  t[name] = value
  return t
end

-- ============================================================
-- Backend process lifecycle
-- ============================================================

function start_topper()
  if topper == nil or not topper.isRunning() then
    topper_now = math.floor(os.clock() * 100)
    feeds = {}
    last_feed_value = {}
    feeds["predict"] = true
    feeds["defend"] = true
    feeds["firstaid"] = true
    topper = spawn(read_topper,
      HOME_DIR .. [[git\topper\target\release\topper-aetolia-mudlet.exe]],
      HOME_DIR .. [[topper_logs]],
      HOME_DIR .. [[git\topper\topper.db-fixed]],
      HOME_DIR .. [[git\topper\triggers]],
      HOME_DIR .. [[git\topper\behavior_trees]],
      HOME_DIR .. [[git\topper\aff_stacks]],
      HOME_DIR .. [[git\topper\topper-explainer\my_logs\]])
  else
    echo("Topper already running!\n")
  end
end

function stop_topper()
  if topper ~= nil then
    if topper.isRunning() then
      write_topper("Kill")
    end
  end
end

function force_stop_topper()
  if topper ~= nil then
    if topper.isRunning() then
      topper.close()
    end
  end
end

-- ============================================================
-- Inbound: decoding backend responses
-- ============================================================

-- Indirection allows easier development/hot-reload of handle_topper.
function read_topper(value)
  handle_topper(value)
end

function handle_topper(value)
  local start = os.clock()
  if string.byte(value) ~= string.byte("{") then
    cecho("<purple>CRASH: " .. value)
    return
  end
  local response = JSON:decode(value)
  if response["battle_stats"] ~= nil then
    topStats = response.battle_stats
  end
  if response["qeb"] ~= nil then
    smart_attack = response["qeb"]
    from_topper = true
    for command in string.gmatch(smart_attack, "[^\r\n]+") do
      if #command > 0 then
        expandAlias(command)
      end
    end
    from_topper = false
  end
  if response["passive"] ~= nil then
    for feed, value in pairs(response["passive"]) do
      if feeds[feed] then
        if value ~= "" then
          expandAlias(value)
        end
        last_feed_value[feed] = value
      else
        if value ~= "" then
          echo("Feed " .. feed .. ": " .. value)
        end
        last_feed_value[feed] = value
      end
    end
  end
  if response["error"] ~= nil then
    cecho("<red>Topper: " .. response["error"])
  end
  if os.clock() - start > 0.02 then
    cecho("<magenta>\\<--" .. (os.clock() - start) .. "--\x3E")
  end
end

-- ============================================================
-- Outbound: encoding and sending messages to the backend
-- ============================================================

function write_topper(data)
  if topper ~= nil then
    if topper.isRunning() then
      local json = JSON:encode(data):gsub([["None"]], "null")
      topper.send(json .. "\n")
    end
  end
end

-- ============================================================
-- Generic observation-tagging API
--
-- These functions are the contract that *other* (content-specific) Mudlet
-- triggers elsewhere in the profile use to attach structured Observations
-- to the TimeSlice this script builds and sends. They don't parse any game
-- text themselves.
-- 
-- Adding new triggers to the observation system should be preferred over
-- injecting them like this.
-- ============================================================


function add_observation(observation)
  if observations == nil then
    observations = {}
  end
  observations[#observations + 1] = observation
end

function observe(type, first, second)
  if first == "You" or first == "you" then
    first = gmcp.Char.Name.name
  end
  if second == "You" or second == "you" then
    second = gmcp.Char.Name.name
  end
  if second ~= nil then
    local observation = {}
    observation[1] = first
    observation[2] = second
    add_observation(tagged(type, observation))
  elseif first ~= nil then
    add_observation(tagged(type, first))
  else
    add_observation(type)
  end
end

local function de_you(who)
  if who == "You" or who == "you" or who == "Your" or who == "your" then
    who = gmcp.Char.Name.name
  elseif who == nil then
    who = ""
  end
  return who
end

function begin_combat_message(caster, category, skill, annotation, target)
  caster = de_you(caster)
  target = de_you(target)
  if annotation == nil then
    annotation = ""
  end
  local combat_message = {}
  combat_message["caster"] = caster
  combat_message["category"] = category
  combat_message["skill"] = skill
  combat_message["annotation"] = annotation
  combat_message["target"] = target
  add_observation(tagged("CombatAction", combat_message))
end

function begin_proc(caster, category, skill, annotation, target)
  caster = de_you(caster)
  target = de_you(target)
  if annotation == nil then
    annotation = ""
  end
  local combat_message = {}
  combat_message["caster"] = caster
  combat_message["category"] = category
  combat_message["skill"] = skill
  combat_message["annotation"] = annotation
  combat_message["target"] = target
  add_observation(tagged("Proc", combat_message))
end

-- ============================================================
-- Raw line / GMCP / sent-command collection
-- ============================================================

function observe_line(text, line_number)
  if topper_lines == nil then
    topper_lines = {}
  end
  topper_lines[#topper_lines + 1] = { text, line_number }
end

-- Optional escape hatch: define topper_should_skip_line() elsewhere in the
-- profile to suppress line capture under specific conditions (e.g. during a
-- spammy sequence, where timing matters) without coupling this bridge to
-- that feature.
function send_line(echo)
  if type(topper_should_skip_line) == "function" and topper_should_skip_line() then
    return
  end
  if echo then
    moveCursorEnd()
    moveCursorUp()
  end
  selectCurrentLine()
  local text = getSelection()
  observe_line(text .. "\r\n", getLineNumber())
end

-- echo()s a line to the console *and* feeds it into the captured lines for
-- the backend, for locally-generated text (e.g. status messages) that never
-- came from the game and so wouldn't otherwise be observed.
function full_echo(text)
  echo(text)
  send_line()
end

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

function observe_send(command)
  if topper_now == nil then
    return
  end
  local time_slice = {}
  time_slice["prompt"] = "Promptless"
  time_slice["observations"] = { tagged("Sent", command) }
  time_slice["lines"] = {}
  time_slice["gmcp"] = {}
  time_slice["time"] = math.floor(os.clock() * 100) - topper_now
  time_slice["me"] = gmcp.Char.Name.name
  write_topper(tagged("TimeSlice", time_slice))
end

function watch_data(eventName, commandSent)
  observe_send(commandSent)
end

-- ============================================================
-- TimeSlice assembly and dispatch
-- ============================================================

function build_prompt_stats(vitals)
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

function build_time_slice(prompt)
  local time_slice = {}
  time_slice["prompt"] = prompt
  time_slice["observations"] = observations or {}
  time_slice["lines"] = topper_lines or {}
  time_slice["gmcp"] = topper_gmcp or {}
  time_slice["time"] = math.floor(os.clock() * 100) - topper_now
  time_slice["me"] = gmcp.Char.Name.name
  return time_slice
end

function send_time_slice(vitals)
  if vitals == nil or topper_now == nil then
    return
  end
  local prompt = tagged("Stats", build_prompt_stats(vitals))
  write_topper(tagged("TimeSlice", build_time_slice(prompt)))
  observations = {}
  topper_lines = {}
  topper_gmcp = {}
end

-- ============================================================
-- Request API
--
-- Thin wrappers around the TopperRequest JSON contract
-- (see topper-core-mudlet/src/topper/mod.rs). Used by command aliases
-- and other scripts elsewhere in the profile.
-- ============================================================

function send_request(type, value)
  if value ~= nil then
    write_topper(tagged("Request", tagged(type, value)))
  else
    write_topper(tagged("Request", type))
  end
end

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

function request_module(module, command)
  send_request("ModuleMsg", { module, command })
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
-- Script-created triggers / event handlers
--
-- Core.lua does not rely on any GUI-created Trigger/Script existing in the
-- profile; it creates everything it needs here, and guards against
-- duplicate registration if this file is reloaded.
-- ============================================================

-- Fires once per received line: feeds every line of game text into the
-- TimeSlice's "lines" and clears the catch/tag state left by any
-- content-specific trigger that classified it (equivalent to the exported
-- "Topper End" catch-all trigger).
function topper_catch_all_line()
  send_line()
  if is_caught() then
    end_line()
  end
end

-- Fires once per prompt: this is the heartbeat that actually sends the
-- accumulated TimeSlice to the backend (equivalent to the exported
-- "Prompt Found" trigger).
function topper_on_prompt()
  if gmcp.Char ~= nil then
    send_time_slice(gmcp.Char.Vitals)
  end
end

local TOPPER_GMCP_EVENTS = {
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

local function install_topper_core_triggers()
  if topper_core_catch_all_trigger_id then
    killTrigger(topper_core_catch_all_trigger_id)
  end
  if topper_core_prompt_trigger_id then
    killTrigger(topper_core_prompt_trigger_id)
  end

  topper_core_catch_all_trigger_id = tempRegexTrigger("^(.*)$", topper_catch_all_line)
  topper_core_prompt_trigger_id = tempPromptTrigger(topper_on_prompt)

  -- registerAnonymousEventHandler is a no-op if the same (event, function)
  -- pair is already registered, so these are safe to call again on reload.
  for _, event in ipairs(TOPPER_GMCP_EVENTS) do
    registerAnonymousEventHandler(event, "handle_gmcp")
  end
  registerAnonymousEventHandler("sysDataSendRequest", "watch_data")
end

install_topper_core_triggers()

start_topper()
