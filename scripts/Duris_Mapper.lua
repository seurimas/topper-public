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

-- Loads a zone map, replacing any existing area with the same name.
function duris_load_zone(payload)
  local data, err = decode_payload(payload)
  if data == nil then
    cecho("<red>Map: " .. err .. "\n")
    return false
  end

  local nodes = data.map.nodes or {}
  local edges = data.map.edges or {}
  local area_name = "Zone " .. tostring(data.zone_id or "?")
  if type(data.info) == "table" and type(data.info.name) == "string" then
    area_name = clean_name(data.info.name)
  end

  local existing = getAreaTable()[area_name]
  if existing then
    deleteArea(existing)
  end
  local area_id = addAreaName(area_name)

  -- Create rooms; topper node id -> Mudlet room id.
  local room_by_node = {}
  for _, node in ipairs(nodes) do
    local room_id = createRoomID()
    room_by_node[node.id] = room_id
    addRoom(room_id, area_id)
    setRoomName(room_id, clean_name(node.name))
    setRoomArea(room_id, area_id)
    setRoomUserData(room_id, "topper_id", tostring(node.id))
    if node.sectorType ~= nil then
      setRoomUserData(room_id, "topper_sector", tostring(node.sectorType))
    end
  end

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

  for _, node in ipairs(nodes) do
    local c = coords[node.id]
    setRoomCoordinates(room_by_node[node.id], c[1], c[2], c[3])
  end

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

  local player_target
  local current_num = gmcp and gmcp.Room and gmcp.Room.Info and gmcp.Room.Info.num
  if current_num ~= nil then
    for _, node in ipairs(nodes) do
      if tostring(node.id) == tostring(current_num) then
        player_target = room_by_node[node.id]
        break
      end
    end
  end
  if player_target then
    setPlayerRoom(player_target)
    centerview(player_target)
  elseif #nodes > 0 then
    centerview(room_by_node[nodes[1].id])
  end

  updateMap()
  saveMap()

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
