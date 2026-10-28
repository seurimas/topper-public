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

  It also does NOT include anything specific to the Aetolia build of the
  backend (concrete TopperRequest wrappers, Aetolia's GMCP schema, how to
  read Aetolia's vitals into the "Stats" prompt variant) -- see Aetolia.lua,
  which must be loaded after this file and supplies those pieces.

  Nothing here depends on any pre-existing GUI-created Trigger/Alias/Script
  from the exported profile; every Mudlet trigger and event handler this
  bridge needs is created by this script when it loads.
]]

-- ============================================================
-- Configuration
-- ============================================================

-- Root of the user's home directory (e.g. "C:\Users\<user>\"); all backend
-- paths below (the running executable, its logs/db/triggers/behavior-trees/
-- aff-stacks directories) are relative to this, same as before.
--
-- Mudlet's getMudletHomeDir() returns the current *profile's* directory
-- instead, e.g. "<HOME_DIR>.config/mudlet/profiles/<ProfileName>", so we
-- recover HOME_DIR by stripping that ".config/mudlet/profiles/<ProfileName>"
-- suffix back off -- this works for any profile name, without needing one
-- hardcoded here.
local mudletHomeDir = getMudletHomeDir()
HOME_DIR = (mudletHomeDir:gsub("[\\/]%.config[\\/]mudlet[\\/]profiles[\\/][^\\/]+[\\/]?$", "")) .. "/"

-- ============================================================
-- Wire format (JSON)
-- ============================================================
-- The backend expects line-delimited JSON. We reuse the same JSON library
-- the original profile used (rather than Mudlet's built-in yajl) because its
-- encoding of empty tables/booleans and the "None" sentinel substitution
-- below are already tuned to match what serde_json expects on the Rust side.
-- JSON.lua lives alongside the profile itself, so it's located via
-- getMudletHomeDir() directly (not HOME_DIR) -- this also works for any
-- profile name.
if JSON == nil then
  local jsonPath = mudletHomeDir .. "/JSON.lua"
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
-- GMCP snapshot
-- ============================================================

-- Each game module sets these to its own GMCP paths (see Aetolia.lua, Duris.lua).
GMCP_VITALS_PATH = GMCP_VITALS_PATH or "gmcp.Char.Vitals"
GMCP_NAME_PATH = GMCP_NAME_PATH or "gmcp.Char.Name"

-- Resolves "gmcp.Char.Vitals" to gmcp.Char.Vitals by walking the dotted path.
function gmcp_payload(name)
  local node = gmcp
  for part in string.gmatch(name, "[^%.]+") do
    if part ~= "gmcp" then
      if type(node) ~= "table" then
        return nil
      end
      node = node[part]
    end
  end
  return node
end

function topper_vitals()
  return gmcp_payload(GMCP_VITALS_PATH)
end

-- The player's name, read from the "name" field of the configured GMCP path.
function topper_me()
  local who = gmcp_payload(GMCP_NAME_PATH)
  if type(who) == "table" and who.name ~= nil then
    return who.name
  end
  return ""
end

-- Sends the current value of each game-declared snapshot event (topper_snapshot_gmcp_events)
-- so the backend starts with GMCP state that arrived before it was launched.
function send_gmcp_snapshot()
  if topper_snapshot_gmcp_events == nil or topper_now == nil then
    return
  end
  local items = {}
  for _, event in ipairs(topper_snapshot_gmcp_events) do
    local payload = gmcp_payload(event)
    if payload ~= nil then
      items[#items + 1] = { event, payload }
    end
  end
  if #items == 0 then
    return
  end
  local time_slice = {}
  time_slice["prompt"] = "Promptless"
  time_slice["observations"] = {}
  time_slice["lines"] = {}
  time_slice["gmcp"] = items
  time_slice["time"] = math.floor(os.clock() * 100) - topper_now
  time_slice["me"] = topper_me()
  write_topper(tagged("TimeSlice", time_slice))
end

-- ============================================================
-- Backend process lifecycle
-- ============================================================

function start_topper(TOPPER_VERSION)
  if topper == nil or not topper.isRunning() then
    topper_now = math.floor(os.clock() * 100)
    topper_version = TOPPER_VERSION
    topper_user_stopped = false
    feeds = {}
    last_feed_value = {}
    feeds["predict"] = true
    feeds["defend"] = true
    feeds["firstaid"] = true
    topper = spawn(read_topper,
      HOME_DIR .. [[git\topper\target\release\topper-]] .. TOPPER_VERSION .. [[-mudlet.exe]],
      HOME_DIR .. [[topper_logs]],
      HOME_DIR .. [[git\topper\topper-]] .. TOPPER_VERSION .. [[\topper.db-fixed]],
      HOME_DIR .. [[git\topper\topper-]] .. TOPPER_VERSION .. [[\triggers]],
      HOME_DIR .. [[git\topper\topper-]] .. TOPPER_VERSION .. [[\behavior_trees]],
      HOME_DIR .. [[git\topper\topper-]] .. TOPPER_VERSION .. [[\aff_stacks]],
      HOME_DIR .. [[git\topper\topper-explainer\my_logs\]])
    send_gmcp_snapshot()
  else
    echo("Topper already running!\n")
  end
end

function stop_topper()
  if topper ~= nil then
    if topper.isRunning() then
      topper_user_stopped = true
      write_topper("Kill")
    end
  end
end

function force_stop_topper()
  if topper ~= nil then
    if topper.isRunning() then
      topper_user_stopped = true
      topper.close()
    end
  end
end

-- ============================================================
-- Build-triggered restart
-- ============================================================

-- Shared with topper-duris-mudlet/build.ps1 and watch_for_build_shutdown().
local TOPPER_CONTROL_DIR = HOME_DIR .. [[git\topper\target\topper-control\]]

function wait_for_build_then_restart()
  tempTimer(1, check_build_then_restart)
end

function check_build_then_restart()
  local done = io.open(TOPPER_CONTROL_DIR .. "build-complete", "r")
  if done then
    done:close()
    os.remove(TOPPER_CONTROL_DIR .. "build-complete")
    start_topper(topper_version)
  else
    wait_for_build_then_restart()
  end
end

-- ============================================================
-- Inbound: decoding backend responses
-- ============================================================

-- Indirection allows easier development/hot-reload of handle_topper.
function read_topper(value)
  handle_topper(value)
end

-- Game modules register handlers here to consume passive feeds instead of echoing them.
topper_passive_handlers = topper_passive_handlers or {}

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
      local handler = topper_passive_handlers[feed]
      if handler then
        handler(value)
      elseif feeds[feed] then
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
  if response["die"] == true and not topper_user_stopped then
    wait_for_build_then_restart()
  end
  if os.clock() - start > 0.02 then
    cecho("<magenta>\\<--" .. (os.clock() - start) .. "--\x3E")
  end
end

-- ============================================================
-- Outbound: encoding and sending messages to the backend
-- ============================================================

-- JSON null arrives from GMCP as userdata, which JSON:encode cannot serialize.
local function topper_sanitize(value)
  local kind = type(value)
  if kind == "userdata" then
    return "None"
  elseif kind == "table" then
    local copy = {}
    for k, v in pairs(value) do
      copy[k] = topper_sanitize(v)
    end
    return copy
  end
  return value
end

function write_topper(data)
  if topper ~= nil then
    if topper.isRunning() then
      local json = JSON:encode(topper_sanitize(data)):gsub([["None"]], "null")
      echo("Sending: " .. json)
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
    first = topper_me()
  end
  if second == "You" or second == "you" then
    second = topper_me()
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
    who = topper_me()
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

-- GMCP capture is game-specific (which paths exist and what they contain
-- varies by MUD/backend) -- see handle_gmcp() in Aetolia.lua, which appends
-- to topper_gmcp the same way this file's observe_line() appends to
-- topper_lines.

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
  time_slice["me"] = topper_me()
  write_topper(tagged("TimeSlice", time_slice))
end

function watch_data(eventName, commandSent)
  observe_send(commandSent)
end

-- ============================================================
-- TimeSlice assembly and dispatch
-- ============================================================

-- build_prompt_stats(vitals) turns the game's vitals into the "Stats"
-- prompt variant the backend expects; what "vitals" looks like is game
-- specific, so the implementation lives in a game module (see
-- build_aetolia_prompt_stats in Aetolia.lua, which this is pointed at).
build_prompt_stats = nil

function build_time_slice(prompt)
  local time_slice = {}
  time_slice["prompt"] = prompt
  time_slice["observations"] = observations or {}
  time_slice["lines"] = topper_lines or {}
  time_slice["gmcp"] = topper_gmcp or {}
  time_slice["time"] = math.floor(os.clock() * 100) - topper_now
  time_slice["me"] = topper_me()
  return time_slice
end

function send_time_slice(vitals)
  if vitals == nil or topper_now == nil then
    return
  end
  local prompt
  if type(build_prompt_stats) == "function" then
    prompt = tagged("Stats", build_prompt_stats(vitals))
  else
    prompt = "Promptless"
  end
  write_topper(tagged("TimeSlice", build_time_slice(prompt)))
  observations = {}
  topper_lines = {}
  topper_gmcp = {}
end

-- ============================================================
-- Request API
--
-- Generic wrappers around the TopperRequest JSON contract
-- (see topper-core-mudlet/src/topper/mod.rs). Concrete request shortcuts
-- (Target/Attack/Plan/Hint/...) are backend-specific and live in a game
-- module such as Aetolia.lua.
-- ============================================================

function send_request(type, value)
  if value ~= nil then
    write_topper(tagged("Request", tagged(type, value)))
  else
    write_topper(tagged("Request", type))
  end
end

function request_module(module, command)
  send_request("ModuleMsg", { module, command })
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
end

-- Fires once per prompt: this is the heartbeat that actually sends the
-- accumulated TimeSlice to the backend (equivalent to the exported
-- "Prompt Found" trigger).
function topper_on_prompt()
  send_time_slice(topper_vitals())
end

local function install_topper_core_triggers()
  if topper_core_catch_all_trigger_id then
    killTrigger(topper_core_catch_all_trigger_id)
  end
  if topper_core_prompt_trigger_id then
    killTrigger(topper_core_prompt_trigger_id)
  end

  topper_core_catch_all_trigger_id = tempRegexTrigger("^(.*)$", topper_catch_all_line)
  topper_core_prompt_trigger_id = tempPromptTrigger(topper_on_prompt)

  -- GMCP event handlers are game-specific (which paths exist varies by
  -- MUD/backend) and are registered by a game module instead, e.g.
  -- install_aetolia_gmcp_handlers() in Aetolia.lua.
  registerAnonymousEventHandler("sysDataSendRequest", "watch_data")
end

-- ============================================================
-- Named aliases (Topper group)
-- ============================================================

-- Registry of user-facing commands, rebuilt on each install so reloads don't duplicate entries.
local topper_commands = {}

-- Creates the named alias if missing and records it for the `topper` listing.
function register_topper_command(name, pattern, code, usage, description)
  topper_commands[#topper_commands + 1] = { usage = usage, description = description }
  if exists(name, "alias") == 0 then
    permAlias(name, "Topper", pattern, code)
  end
end

function list_topper_commands()
  echo("Topper commands:\n")
  for _, command in ipairs(topper_commands) do
    echo(string.format("  %-12s %s\n", command.usage, command.description))
  end
end

local function install_topper_core_aliases()
  if exists("Topper", "alias") == 0 then
    permGroup("Topper", "alias")
  end
  topper_commands = {}
  register_topper_command("List", "^topper$", "list_topper_commands()", "topper", "List known topper commands")
  register_topper_command("Startup", "^topper on$", "start_topper(topper_version)", "topper on", "Start topper")
  register_topper_command("Shutdown", "^topper off$", "stop_topper()", "topper off", "Stop topper")
  register_topper_command("Debug", "^tdebug$", "request_module('core', 'debug')", "tdebug", "Toggle backend debug mode")
end

install_topper_core_triggers()
install_topper_core_aliases()