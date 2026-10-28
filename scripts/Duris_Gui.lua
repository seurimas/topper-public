-- Three-column Duris layout: comms (left), main display (center), tabs and map (right), stat bars (bottom).

local BOTTOM_BAR_HEIGHT = 40
local SIDE_WIDTH = 400
local TAB_NAMES = {"Mobs", "Equip", "World Map"}
local TAB_STYLE = [[background-color: rgb(40,40,40); border-radius: 6px; margin: 1px;]]
local TAB_ACTIVE_STYLE = [[background-color: rgb(80,80,80); border-radius: 6px; margin: 1px;]]
local TAB_BODY_STYLE = [[background-color: rgb(20,20,20);]]

duris_gui = {}
local gui = duris_gui

local function build_bottom_bars()
  local bar = Geyser.Container:new({
    name = "DurisBottomBar",
    x = 0, y = -BOTTOM_BAR_HEIGHT,
    width = "100%", height = BOTTOM_BAR_HEIGHT,
  })
  local colors = {
    health  = {200, 30, 30},
    mana    = {30, 90, 220},
    stamina = {220, 200, 30},
  }
  local order = {"health", "mana", "stamina"}
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

local function build_comms()
  gui.comms = Geyser.MiniConsole:new({
    name = "DurisComms",
    x = 0, y = 0,
    width = SIDE_WIDTH, height = "100%",
  }, gui.left)
  gui.comms:setWrap(60)
end

local function select_tab(tab)
  for _, name in ipairs(TAB_NAMES) do
    gui.tabPages[name]:hide()
    gui.tabLabels[name]:setStyleSheet(TAB_STYLE)
  end
  gui.tabPages[tab]:show()
  gui.tabLabels[tab]:setStyleSheet(TAB_ACTIVE_STYLE)
  gui.currentTab = tab
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
    x = 0, y = "50%",
    width = "100%", height = "50%",
  }, gui.right)
  gui.map = Geyser.Mapper:new({
    name = "DurisMapper",
    x = 0, y = 0,
    width = "100%", height = "100%",
  }, gui.mapHolder)
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

  build_comms()
  build_right_panel()
  build_bottom_bars()
end

function duris_select_tab(name)
  select_tab(name)
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
