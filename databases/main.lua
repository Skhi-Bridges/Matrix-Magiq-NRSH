-- distance-metrics/main.lua

local DistanceMetrics = {}
local LogFile = "logs/metrics_log.txt"

-- Logging function
local function WriteLog(message)
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    local file = io.open(LogFile, "a")
    file:write(timestamp .. " - " .. message .. "\n")
    file:close()
end

-- Basic Metrics
DistanceMetrics.euclidean = function(a, b)
    local sum = 0
    for i = 1, #a do
        sum = sum + (a[i] - b[i])^2
    end
    return math.sqrt(sum)
end

DistanceMetrics.manhattan = function(a, b)
    local sum = 0
    for i = 1, #a do
        sum = sum + math.abs(a[i] - b[i])
    end
    return sum
end

DistanceMetrics.cosine = function(a, b)
    local dot_product = 0
    local norm_a = 0
    local norm_b = 0
    for i = 1, #a do
        dot_product = dot_product + a[i] * b[i]
        norm_a = norm_a + a[i]^2
        norm_b = norm_b + b[i]^2
    end
    return 1 - (dot_product / (math.sqrt(norm_a) * math.sqrt(norm_b)))
end

-- All other functions as provided...

return DistanceMetrics
