-- Builds a Mudlet area for the current Duris zone from a topper map payload.
-- Payload: { zone_id, info, map = { nodes = [{id, name, sectorType}], edges = [{from, to, direction}] } }

local DIRECTIONS = {
  north = {0, 1, 0}, south = {0, -1, 0}, east = {1, 0, 0}, west = {-1, 0, 0},
  northeast = {1, 1, 0}, northwest = {-1, 1, 0}, southeast = {1, -1, 0}, southwest = {-1, -1, 0},
  up = {0, 0, 1}, down = {0, 0, -1},
}

local DIRECTION_ALIASES = {
  n = "north", s = "south", e = "east", w = "west",
  ne = "northeast", nw = "northwest", se = "southeast", sw = "southwest",
  u = "up", d = "down",
}

local function normalize_direction(dir)
  local lower = string.lower(tostring(dir or ""))
  return DIRECTION_ALIASES[lower] or lower
end

-- Strips Duris color codes (&+X, &-X, &n) from a name.
local function clean_name(name)
  local s = tostring(name or "")
  s = s:gsub("&[%+%-]%a", ""):gsub("&n", "")
  return s
end

local function decode_payload(payload)
  if type(payload) == "string" then
    local ok, decoded = pcall(function() return JSON:decode(payload) end)
    if not ok then
      return nil, "invalid JSON"
    end
    payload = decoded
  end
  if type(payload) ~= "table" or type(payload.map) ~= "table" then
    return nil, "payload has no map"
  end
  return payload
end

local syncing_room = false
local last_synced_vnum = nil

-- Moves the Mudlet player to the room whose id is the game vnum.
-- The guard stops sysManualLocationSetEvent from re-entering map.find_me.
local function sync_player_room(vnum)
  local id = tonumber(vnum)
  if id == nil or not roomExists(id) then
    return false
  end
  if syncing_room then
    return true
  end
  syncing_room = true
  local ok, err = pcall(function()
    if id ~= last_synced_vnum then
      raiseEvent("sysManualLocationSetEvent", id)
      centerview(id)
      last_synced_vnum = id
    end
  end)
  syncing_room = false
  if not ok then
    error(err, 0)
  end
  return true
end

-- Fires on every Room.Info so the position follows the game, not name matching.
function duris_sync_player_room()
  local info = gmcp and gmcp.Room and gmcp.Room.Info
  if info then
    sync_player_room(info.num)
  end
end
registerAnonymousEventHandler("gmcp.Room.Info", "duris_sync_player_room")

-- Keeps the generic mapper's find_me so it can be used when GMCP has no vnum.
map = map or {}
duris_generic_find_me = duris_generic_find_me or map.find_me

-- Replaces the generic name-based room lookup whenever GMCP reports a vnum.
function map.find_me(name, exits, dir, manual)
  local info = gmcp and gmcp.Room and gmcp.Room.Info
  if info and info.num ~= nil then
    -- Overworld rooms report vnum 0, which is never mapped.
    if tonumber(info.num) == 0 then
      return
    end
    if not sync_player_room(info.num) then
      cecho("<red>Map: vnum " .. tostring(info.num) .. " is not in the loaded map\n")
    end
    return
  end
  if duris_generic_find_me then
    return duris_generic_find_me(name, exits, dir, manual)
  end
end

local function report_phase(label, since)
  local now = os.clock()
  cecho(string.format("<grey>Map timing: %s %.1f ms\n", label, (now - since) * 1000))
  return now
end

-- Loads a zone map, replacing any existing area with the same name.
function duris_load_zone(payload)
  local t = os.clock()
  local data, err = decode_payload(payload)
  if data == nil then
    cecho("<red>Map: " .. err .. "\n")
    return false
  end

  t = report_phase("decode", t)
  local nodes = data.map.nodes or {}
  local edges = data.map.edges or {}
  local area_name = "Zone " .. tostring(data.zone_id or "?")
  if type(data.info) == "table" and type(data.info.name) == "string" then
    area_name = clean_name(data.info.name)
  end

  -- Rooms use the vnum as their Mudlet id, so every id must be numeric and free.
  local existing = getAreaTable()[area_name]
  if existing then
    cecho("<grey>Map: " .. area_name .. " is already mapped, skipping\n")
    return true
  end
  for _, node in ipairs(nodes) do
    local room_id = tonumber(node.id)
    if room_id == nil then
      cecho("<red>Map: node id '" .. tostring(node.id) .. "' is not a vnum\n")
      return false
    end
    if roomExists(room_id) and getRoomArea(room_id) ~= existing then
      cecho("<red>Map: vnum " .. room_id .. " already exists in another area\n")
      return false
    end
  end
  t = report_phase("validate", t)
  if duris_gui and duris_gui.mapHolder then
    duris_gui.mapHolder:hide()
  end
  if existing then
    deleteArea(existing)
  end
  last_synced_vnum = nil
  local area_id = addAreaName(area_name)
  t = report_phase("delete old area and create area", t)

  -- Create rooms; topper node id -> Mudlet room id (the vnum).
  local room_by_node = {}
  for _, node in ipairs(nodes) do
    local room_id = tonumber(node.id)
    room_by_node[node.id] = room_id
    addRoom(room_id, area_id)
    setRoomName(room_id, clean_name(node.name))
    setRoomArea(room_id, area_id)
    setRoomUserData(room_id, "topper_id", tostring(node.id))
    if node.sectorType ~= nil then
      setRoomUserData(room_id, "topper_sector", tostring(node.sectorType))
    end
  end
  t = report_phase("create rooms", t)

  -- Build an undirected coordinate graph from the directed edges.
  local adjacency = {}
  local function add_step(from, to, delta)
    adjacency[from] = adjacency[from] or {}
    table.insert(adjacency[from], {to = to, delta = delta})
  end
  for _, edge in ipairs(edges) do
    local delta = DIRECTIONS[normalize_direction(edge.direction)]
    if delta and room_by_node[edge.from] and room_by_node[edge.to] then
      add_step(edge.from, edge.to, delta)
      add_step(edge.to, edge.from, {-delta[1], -delta[2], -delta[3]})
    end
  end

  -- BFS per connected component; components are laid out side by side on x.
  local coords = {}
  local next_offset = 0
  local collisions = 0
  local occupied = {}
  for _, start in ipairs(nodes) do
    if coords[start.id] == nil then
      coords[start.id] = {0, 0, 0}
      local queue, head = {start.id}, 1
      while head <= #queue do
        local current = queue[head]
        head = head + 1
        local base = coords[current]
        for _, step in ipairs(adjacency[current] or {}) do
          if coords[step.to] == nil then
            coords[step.to] = {base[1] + step.delta[1], base[2] + step.delta[2], base[3] + step.delta[3]}
            queue[#queue + 1] = step.to
          end
        end
      end

      local min_x, max_x = math.huge, -math.huge
      for _, id in ipairs(queue) do
        min_x = math.min(min_x, coords[id][1])
        max_x = math.max(max_x, coords[id][1])
      end
      local shift = next_offset - min_x
      for _, id in ipairs(queue) do
        local c = coords[id]
        c[1] = c[1] + shift
        local key = c[1] .. "," .. c[2] .. "," .. c[3]
        if occupied[key] then
          collisions = collisions + 1
        end
        occupied[key] = true
      end
      next_offset = next_offset + (max_x - min_x) + 3
    end
  end

  t = report_phase("build adjacency and BFS layout", t)
  for _, node in ipairs(nodes) do
    local c = coords[node.id]
    setRoomCoordinates(room_by_node[node.id], c[1], c[2], c[3])
  end
  t = report_phase("set coordinates", t)

  -- Exits are set from the original directed edges so one-way exits stay one-way.
  local skipped = 0
  for _, edge in ipairs(edges) do
    local dir = normalize_direction(edge.direction)
    if DIRECTIONS[dir] and room_by_node[edge.from] and room_by_node[edge.to] then
      setExit(room_by_node[edge.from], room_by_node[edge.to], dir)
    else
      skipped = skipped + 1
    end
  end
  t = report_phase("set exits", t)

  local current_num = gmcp and gmcp.Room and gmcp.Room.Info and gmcp.Room.Info.num
  if not sync_player_room(current_num) and #nodes > 0 then
    centerview(room_by_node[nodes[1].id])
  end

  updateMap()
  t = report_phase("updateMap", t)
  saveMap()
  t = report_phase("saveMap", t)
  if duris_gui and duris_gui.mapHolder then
    duris_gui.mapHolder:show()
  end

  cecho(string.format("<green>Map: loaded %d rooms, %d exits into '%s'\n", #nodes, #edges - skipped, area_name))
  if skipped > 0 then
    cecho(string.format("<yellow>Map: skipped %d edges with unknown direction or missing room\n", skipped))
  end
  if collisions > 0 then
    cecho(string.format("<yellow>Map: %d rooms overlap in the generated layout\n", collisions))
  end
  return true
end

local MAP_WIDGET_WIDTH = 400
local MAP_WIDGET_HEIGHT = 300
