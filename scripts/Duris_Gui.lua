-- Three-column Duris layout: comms (left), main display (center), tabs and map (right), stat bars (bottom).

local BOTTOM_BAR_HEIGHT = 40
local STATUS_HEIGHT = 22
local SIDE_WIDTH = 400
local GROUP_HEIGHT = 300
local LEFT_TAB_HEIGHT = 30
local LEFT_TAB_NAMES = {"Communication", "Help"}
local TAB_NAMES = {"Mobs", "Equip", "World Map"}
local TAB_STYLE = [[background-color: rgb(40,40,40); border-radius: 6px; margin: 1px;]]
local TAB_ACTIVE_STYLE = [[background-color: rgb(80,80,80); border-radius: 6px; margin: 1px;]]
local TAB_BODY_STYLE = [[background-color: rgb(20,20,20);]]
local COMPASS_HEIGHT = 144
local COMPASS_CELL = 48
local COMPASS_ON_STYLE = [[background-color: rgb(60,60,90); border-radius: 4px; margin: 1px;]]
local COMPASS_OFF_STYLE = [[background-color: rgb(45,45,45); color: rgb(110,110,110); border-radius: 4px; margin: 1px;]]
local OVERLAND_BASE_FONT = 10
local OVERLAND_MAX_FONT = 14

duris_gui = {}
local gui = duris_gui

local function build_bottom_bars()
  local bar = Geyser.Container:new({
    name = "DurisBottomBar",
    x = 0, y = -BOTTOM_BAR_HEIGHT,
    width = "100%", height = BOTTOM_BAR_HEIGHT,
  })
  local colors = {
    health   = {200, 30, 30},
    mana     = {30, 90, 220},
    movement = {220, 200, 30},
    exp      = {160, 80, 220},
  }
  local order = {"health", "mana", "movement", "exp"}
  gui.bars = {}
  for i, key in ipairs(order) do
    local gauge = Geyser.Gauge:new({
      name = "Duris" .. key:gsub("^%l", string.upper) .. "Bar",
      x = ((i - 1) * 100 / #order) .. "%", y = 0,
      width = (100 / #order) .. "%", height = "100%",
    }, bar)
    gauge:setColor(unpack(colors[key]))
    gauge:setValue(100, 100, (key:gsub("^%l", string.upper)))
    gui.bars[key] = gauge
  end
  gui.bottomBar = bar
end

-- Updates a bar's fill and label; max <= 0 leaves the bar empty.
local function set_bar(key, current, max, label)
  local gauge = gui.bars and gui.bars[key]
  if gauge == nil or current == nil or max == nil or max <= 0 then
    return
  end
  gauge:setValue(current, max, label .. ": " .. current .. "/" .. max)
end

-- Redraws the bottom bars from a gmcp.Char.Vitals table. Exp shows progress toward the next level.
function duris_on_vitals()
  local v = gmcp and gmcp.Char and gmcp.Char.Vitals
  if type(v) ~= "table" then
    return
  end
  set_bar("health", tonumber(v.hp), tonumber(v.maxHp), "HP")
  set_bar("mana", tonumber(v.mana), tonumber(v.maxMana), "Mana")
  set_bar("movement", tonumber(v.move), tonumber(v.maxMove), "Movement")
  local exp, tnl = tonumber(v.exp), tonumber(v.tnl)
  if exp ~= nil and tnl ~= nil then
    set_bar("exp", exp, exp + tnl, "Exp")
  end
end

local function select_left_tab(tab)
  for _, name in ipairs(LEFT_TAB_NAMES) do
    gui.leftTabPages[name]:hide()
    gui.leftTabLabels[name]:setStyleSheet(TAB_STYLE)
  end
  gui.leftTabPages[tab]:show()
  gui.leftTabLabels[tab]:setStyleSheet(TAB_ACTIVE_STYLE)
end

function duris_select_left_tab(name)
  select_left_tab(name)
end

local function build_left_panel()
  gui.group = Geyser.Label:new({
    name = "DurisGroup",
    x = 0, y = 0,
    width = "100%", height = GROUP_HEIGHT,
  }, gui.left)
  gui.group:setStyleSheet(TAB_BODY_STYLE)
  gui.group:echo("<center>Group")

  gui.leftTabHeader = Geyser.HBox:new({
    name = "DurisLeftTabHeader",
    x = 0, y = GROUP_HEIGHT,
    width = "100%", height = LEFT_TAB_HEIGHT,
  }, gui.left)
  local bodyTop = GROUP_HEIGHT + LEFT_TAB_HEIGHT
  gui.leftTabBody = Geyser.Label:new({
    name = "DurisLeftTabBody",
    x = 0, y = bodyTop,
    width = "100%", height = "100%-" .. bodyTop .. "px",
  }, gui.left)
  gui.leftTabBody:setStyleSheet(TAB_BODY_STYLE)

  gui.leftTabLabels = {}
  gui.leftTabPages = {}
  for _, name in ipairs(LEFT_TAB_NAMES) do
    local label = Geyser.Label:new({
      name = "DurisLeftTab" .. string.gsub(name, " ", ""),
    }, gui.leftTabHeader)
    label:echo("<center>" .. name)
    label:setClickCallback("duris_select_left_tab", name)
    gui.leftTabLabels[name] = label

    local page = Geyser.Label:new({
      name = "DurisLeftPage" .. string.gsub(name, " ", ""),
      x = 0, y = 0,
      width = "100%", height = "100%",
    }, gui.leftTabBody)
    page:setStyleSheet(TAB_BODY_STYLE)
    page:hide()
    gui.leftTabPages[name] = page
  end
end

local function build_comms()
  gui.comms = Geyser.MiniConsole:new({
    name = "DurisComms",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.leftTabPages["Communication"])
  gui.comms:setWrap(60)
end

local function build_help()
  gui.help = Geyser.MiniConsole:new({
    name = "DurisHelp",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.leftTabPages["Help"])
end

local MOBS_FONT_SIZE = 14
local TARGET_BAR_HEIGHT = 30

-- Renders an autobash payload on the status label.
local function render_autobash(payload)
  local state = payload.state
  local text
  if state == "fighting" then
    text = "Autobash: Fighting " .. (payload.target or "?")
  elseif state == "looting" then
    text = "Autobash: Looting " .. (payload.target or "?")
  elseif state == "resting" then
    text = "Autobash: Resting..."
  elseif state == "engaging" then
    text = "Autobash: Engaging in " .. (payload.ticks_left or "?") .. " ticks"
  elseif state == "moving" then
    text = "Autobash: Moving..."
  elseif state == "idle" then
    local reason = payload.reason or ""
    text = "Autobash: Idle" .. (reason ~= "" and " - " .. reason or "")
  else
    text = "Autobash off"
  end
  gui.autobashStatus:echo("<center><white>" .. text)
end

-- Stacks the target bar (when fighting) above the mobs console, with the autobash status line at the bottom.
local function layout_mobs_tab()
  local top = 0
  if gui.targetShown then
    gui.targetBar:move(0, top)
    top = top + TARGET_BAR_HEIGHT
  end
  gui.mobs:move(0, top)
  gui.mobs:resize("100%", "100%-" .. (top + STATUS_HEIGHT) .. "px")
  gui.autobashStatus:move(0, "100%-" .. STATUS_HEIGHT .. "px")
end

local function build_mobs_console()
  gui.targetBar = Geyser.Gauge:new({
    name = "DurisTargetBar",
    x = 0, y = 0,
    width = "100%", height = TARGET_BAR_HEIGHT,
  }, gui.tabPages["Mobs"])
  gui.targetBar:setColor(160, 30, 30)
  gui.targetBar:hide()
  gui.autobashStatus = Geyser.Label:new({
    name = "DurisAutobashStatus",
    x = 0, y = 0,
    width = "100%", height = STATUS_HEIGHT,
  }, gui.tabPages["Mobs"])
  gui.autobashStatus:echo("<center><dim_grey>Autobash off")
  if gui.autobashLast then
    render_autobash(gui.autobashLast)
  end
  gui.mobs = Geyser.MiniConsole:new({
    name = "DurisMobs",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.tabPages["Mobs"])
  setFontSize(gui.mobs.name, MOBS_FONT_SIZE)
  layout_mobs_tab()
end

local EQUIP_FONT_SIZE = 8

local function build_equip_console()
  gui.equip = Geyser.MiniConsole:new({
    name = "DurisEquip",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.tabPages["Equip"])
  setFontSize(gui.equip.name, EQUIP_FONT_SIZE)
  if gui.currentTab ~= "Equip" then
    gui.equip:hide()
  end
end

local function select_tab(tab)
  for _, name in ipairs(TAB_NAMES) do
    gui.tabPages[name]:hide()
    gui.tabLabels[name]:setStyleSheet(TAB_STYLE)
  end
  gui.tabPages[tab]:show()
  gui.tabLabels[tab]:setStyleSheet(TAB_ACTIVE_STYLE)
  gui.currentTab = tab
  if gui.equip then
    if tab == "Equip" then
      gui.equip:show()
    else
      gui.equip:hide()
    end
  end
end

local function build_right_panel()
  gui.tabHeader = Geyser.HBox:new({
    name = "DurisTabHeader",
    x = 0, y = 0,
    width = "100%", height = "10%",
  }, gui.right)
  gui.tabBody = Geyser.Label:new({
    name = "DurisTabBody",
    x = 0, y = "10%",
    width = "100%", height = "40%",
  }, gui.right)
  gui.tabBody:setStyleSheet(TAB_BODY_STYLE)
  gui.tabLabels = {}
  gui.tabPages = {}
  for _, name in ipairs(TAB_NAMES) do
    local label = Geyser.Label:new({
      name = "DurisTab" .. name:gsub(" ", ""),
    }, gui.tabHeader)
    label:echo("<center>" .. name)
    label:setClickCallback("duris_select_tab", name)
    gui.tabLabels[name] = label

    local page = Geyser.Label:new({
      name = "DurisPage" .. name:gsub(" ", ""),
      x = 0, y = 0,
      width = "100%", height = "100%",
    }, gui.tabBody)
    page:setStyleSheet(TAB_BODY_STYLE)
    page:hide()
    gui.tabPages[name] = page
  end
  select_tab(TAB_NAMES[1])

  gui.mapHolder = Geyser.Container:new({
    name = "DurisMapHolder",
    x = 0, y = "50%+" .. COMPASS_HEIGHT .. "px",
    width = "100%", height = "50%-" .. COMPASS_HEIGHT .. "px",
  }, gui.right)
  gui.map = Geyser.Mapper:new({
    name = "DurisMapper",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.mapHolder)
  gui.overland = Geyser.MiniConsole:new({
    name = "DurisOverland",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.mapHolder)
  setFontSize(gui.overland.name, OVERLAND_BASE_FONT)
  gui.overland:hide()
end

-- Returns true when the exit key is present in gmcp.Room.Info.exits (keyed table or list).
local function exit_active(exits, key)
  if type(exits) ~= "table" then
    return false
  end
  if exits[key] ~= nil and exits[key] ~= false then
    return true
  end
  for _, v in ipairs(exits) do
    if v == key then
      return true
    end
  end
  return false
end

-- Each entry: key sent to the game, label text, and grid row/column (or a side slot).
local COMPASS_GRID = {
  {key = "nw", text = "NW", row = 1, col = 1},
  {key = "n",  text = "N",  row = 1, col = 2},
  {key = "ne", text = "NE", row = 1, col = 3},
  {key = "w",  text = "W",  row = 2, col = 1},
  {key = "e",  text = "E",  row = 2, col = 3},
  {key = "sw", text = "SW", row = 3, col = 1},
  {key = "s",  text = "S",  row = 3, col = 2},
  {key = "se", text = "SE", row = 3, col = 3},
}
local COMPASS_SIDE = {
  {key = "u", text = "Up",   y = 0},
  {key = "d", text = "Down", y = COMPASS_HEIGHT / 2 + 2},
}

-- Click handler: sends the direction only when the exit is active.
function duris_compass_move(key)
  if gui.compassActive and gui.compassActive[key] then
    do_go(key)
  end
end

local function make_compass_button(entry, x, y, w, h)
  local button = Geyser.Label:new({
    name = "DurisCompass" .. entry.key:upper(),
    x = x, y = y,
    width = w, height = h,
  }, gui.compass)
  button:echo("<center>" .. entry.text)
  button:setClickCallback("duris_compass_move", entry.key)
  gui.compassButtons[entry.key] = button
  return button
end

local function build_compass()
  gui.compass = Geyser.Container:new({
    name = "DurisCompass",
    x = 0, y = "50%",
    width = "100%", height = COMPASS_HEIGHT,
  }, gui.right)
  gui.compassButtons = {}
  gui.compassActive = {}
  for _, entry in ipairs(COMPASS_GRID) do
    make_compass_button(entry, (entry.col - 1) * COMPASS_CELL, (entry.row - 1) * COMPASS_CELL,
      COMPASS_CELL - 2, COMPASS_CELL - 2)
  end
  local sideX = 3 * COMPASS_CELL + 8
  for _, entry in ipairs(COMPASS_SIDE) do
    make_compass_button(entry, sideX, entry.y, 80, COMPASS_HEIGHT / 2 - 2)
  end
  duris_update_compass()
end

-- Greys out compass buttons whose exit is not in gmcp.Room.Info.exits.
function duris_update_compass()
  if gui.compassButtons == nil then
    return
  end
  local info = gmcp and gmcp.Room and gmcp.Room.Info
  local exits = info and info.exits
  for key, button in pairs(gui.compassButtons) do
    local active = exit_active(exits, key)
    gui.compassActive[key] = active
    button:setStyleSheet(active and COMPASS_ON_STYLE or COMPASS_OFF_STYLE)
  end
end

-- Shows the autobash state from an autobash data entry as a single status line.
function duris_show_autobash(payload)
  if type(payload) ~= "table" then
    return
  end
  gui.autobashLast = payload
  if gui.autobashStatus == nil then
    return
  end
  render_autobash(payload)
end

function duris_build_gui()
  if gui.built then
    return
  end
  gui.built = true

  setBorderBottom(BOTTOM_BAR_HEIGHT)
  setBorderLeft(SIDE_WIDTH)
  setBorderRight(SIDE_WIDTH)

  gui.left = Geyser.Container:new({
    name = "DurisLeft",
    x = 0, y = 0,
    width = SIDE_WIDTH, height = "100%-" .. BOTTOM_BAR_HEIGHT .. "px",
  })
  gui.right = Geyser.Container:new({
    name = "DurisRight",
    x = "-" .. SIDE_WIDTH .. "px", y = 0,
    width = SIDE_WIDTH, height = "100%-" .. BOTTOM_BAR_HEIGHT .. "px",
  })

  build_left_panel()
  build_comms()
  build_help()
  select_left_tab(LEFT_TAB_NAMES[1])
  build_right_panel()
  build_compass()
  build_mobs_console()
  build_equip_console()
  build_bottom_bars()
end

function duris_select_tab(name)
  select_tab(name)
end

-- Scales the console font so a cols x rows grid fits; returns the resulting cell size in pixels.
local function fit_overland_font(console, cols, rows)
  setFontSize(console.name, OVERLAND_BASE_FONT)
  local cw, ch = calcFontSize(console.name)
  local scale = math.min(console:get_width() / (cols * cw), console:get_height() / (rows * ch))
  setFontSize(console.name, math.max(4, math.min(OVERLAND_MAX_FONT, math.floor(OVERLAND_BASE_FONT * scale))))
  return calcFontSize(console.name)
end

-- Draws the overland map from a map.wilderness payload, fitted to the panel and centered.
local function render_overland()
  local console = gui.overland
  local map = gui.wildernessMap
  if console == nil or map == nil or map.rows == nil or #map.rows == 0 then
    return
  end
  local cols, rows = 0, #map.rows
  for _, row in ipairs(map.rows) do
    cols = math.max(cols, #row)
  end
  if cols == 0 then
    return
  end
  local cw, ch = fit_overland_font(console, cols, rows)
  local visibleCols = math.floor(console:get_width() / cw)
  local visibleRows = math.floor(console:get_height() / ch)
  local topPad = math.max(0, math.floor((visibleRows - rows) / 2))
  local leftPad = string.rep(" ", math.max(0, math.floor((visibleCols - cols) / 2)))

  console:setWrap(visibleCols)
  console:clear()
  for _ = 1, topPad do
    echo(console.name, "\n")
  end
  for _, row in ipairs(map.rows) do
    echo(console.name, leftPad)
    for _, cell in ipairs(row) do
      local fg = cell.fg or {200, 200, 200}
      local bg = cell.bg or {0, 0, 0}
      setFgColor(console.name, fg[1], fg[2], fg[3])
      setBgColor(console.name, bg[1], bg[2], bg[3])
      echo(console.name, cell.ch)
    end
    echo(console.name, "\n")
  end
  resetFormat(console.name)
end

-- Stores the latest overland map and redraws it if it is on screen.
function duris_show_wilderness(payload)
  if type(payload) ~= "table" then
    return
  end
  gui.wildernessMap = payload
  if gui.overlandOn then
    render_overland()
  end
end

-- Re-fits the overland map when the Mudlet window changes size.
function duris_on_resize()
  if gui.overlandOn then
    tempTimer(0, render_overland)
  end
end

-- Swaps the mapper for the overland map while gmcp.Room.Info reports vnum 0.
function duris_update_overland()
  if gui.map == nil or gui.overland == nil then
    return
  end
  local info = gmcp and gmcp.Room and gmcp.Room.Info
  local on = info ~= nil and tonumber(info.num) == 0
  if on == gui.overlandOn then
    return
  end
  gui.overlandOn = on
  if on then
    gui.map:hide()
    gui.overland:show()
    tempTimer(0, render_overland)
  else
    gui.overland:hide()
    gui.map:show()
  end
end

-- Shows the fight target's health bar above the mob list while gmcp.Char.Vitals.Fighting is set.
function duris_update_target()
  if gui.targetBar == nil or gui.mobs == nil then
    return
  end
  local fighting = gmcp and gmcp.Char and gmcp.Char.Vitals and gmcp.Char.Vitals.fighting ~= nil
  if fighting ~= gui.targetShown then
    gui.targetShown = fighting
    if fighting then
      gui.targetBar:show()
    else
      gui.targetBar:hide()
    end
    layout_mobs_tab()
  end
  if not fighting then
    return
  end
  local target = gmcp.Combat and gmcp.Combat.Update and gmcp.Combat.Update.target
  if type(target) ~= "table" then
    gui.targetBar:setValue(0, 100, "No target")
    return
  end
  local pct = math.max(0, math.min(100, tonumber(target.healthPercent) or 0))
  gui.targetBar:setValue(pct, 100, string.format("%s: %s (%.0f%%)", target.name or "?", target.health or "?", pct))
end

-- Redraws the Mobs tab from a mobs.room JSON payload; clicking a mob sends its attack commands.
function duris_show_mobs(payload)
  if gui.mobs == nil or type(payload) ~= "table" then
    return
  end
  local decoded = payload
  gui.mobs:clear()
  if decoded.mobs == nil or #decoded.mobs == 0 then
    gui.mobs:cecho("<dim_grey>No mobs in this room.\n")
    return
  end
  for _, mob in ipairs(decoded.mobs) do
    local level = mob.level and tostring(mob.level) or "?"
    local exp = mob.exp and tostring(mob.exp) or "?"
    local label = string.format("[%s (%s)] %s", level, exp, mob.name or "?")
    if mob.tag then
      local command = string.format("send(%q)", "backstab " .. mob.tag .. ";;hit " .. mob.tag)
      gui.mobs:cechoLink(string.format("<white>%s\n", label), command, "Backstab and hit " .. mob.tag, true)
    else
      gui.mobs:cecho(string.format("<white>%s\n", label))
    end
  end
end

-- Redraws the Equip tab from an equipment JSON payload.
function duris_show_equipment(payload)
  if gui.equip == nil or type(payload) ~= "table" then
    return
  end
  gui.equip:clear()
  if payload.equipment == nil or #payload.equipment == 0 then
    gui.equip:cecho("<dim_grey>Nothing equipped.\n")
    return
  end
  for _, item in ipairs(payload.equipment) do
    gui.equip:cecho(string.format("<white>[%s] %s\n", item.place or "?", item.name or "?"))
    if item.desc and item.desc ~= "" then
      gui.equip:cecho(string.format("<dim_grey> - %s\n", item.desc))
    end
  end
end

-- Attaches each annotation as a tooltip to its history line; payload maps line number to text.
function duris_annotate(payload)
  if type(payload) ~= "table" then
    return
  end
  for key, annotation in pairs(payload) do
    local line = tonumber(key)
    if line ~= nil and type(annotation) == "string" then
      moveCursor(0, line)
      selectCurrentLine()
      setLink("", annotation)
      moveCursorEnd()
    end
  end
end

-- Routes each gmcp.Comm event to the comms console.
function duris_on_comm()
  local comm = gmcp and gmcp.Comm and gmcp.Comm.Channel
  if type(comm) ~= "table" or gui.comms == nil then
    return
  end
  local channel = comm.channel or "?"
  local sender = comm.sender or "?"
  local text = comm.text or ""
  gui.comms:cecho(string.format("<dim_grey>[%s] <white>%s<reset>: %s\n", channel, sender, text))
end

duris_build_gui()
registerAnonymousEventHandler("gmcp.Comm", "duris_on_comm")
registerAnonymousEventHandler("gmcp.Char.Vitals", "duris_on_vitals")
registerAnonymousEventHandler("gmcp.Char.Vitals", "duris_update_target")
registerAnonymousEventHandler("gmcp.Combat.Update", "duris_update_target")
registerAnonymousEventHandler("gmcp.Room.Info", "duris_update_compass")
registerAnonymousEventHandler("gmcp.Room.Info", "duris_update_overland")
registerAnonymousEventHandler("sysWindowResizeEvent", "duris_on_resize")