-- annoy.lua
local M = {}

function M.start(processId)
  -- Initialize Annoy index
  local annoy = {
    dimension = 1024,
    metric = 'angular'
  }

  -- Interface functions
  function M.add_item(i, vector)
    -- Add item to index
  end

  function M.build(n_trees)
    -- Build the index
  end

  function M.get_nns_by_vector(vector, n)
    -- Get nearest neighbors
  end

  return annoy
end

return M
