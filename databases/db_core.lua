local M = {}

-- Load configuration
local config = require('config.db_config')

-- Metrics collector interface
M.metrics = {
    collect = function(db_name, metric_type, value)
        -- Will be connected to ao_processes/metrics
        if package.loaded['ao_processes.metrics'] then
            return require('ao_processes.metrics').record(db_name, metric_type, value)
        end
        return true
    end
}

-- Tuning interface
M.tuning = {
    get_params = function(db_name)
        -- Will be connected to ao_processes/tuning
        if package.loaded['ao_processes.tuning'] then
            return require('ao_processes.tuning').get_parameters(db_name)
        end
        return {}
    end
}

-- Database process base class
function M.create_db_process(name, opts)
    local process = {
        name = name,
        config = config[name] or {},
        options = opts or {},
        _metrics = {},
        _tuning_params = {}
    }

    -- Initialize with current tuning parameters
    process._tuning_params = M.tuning.get_params(name)

    -- Metrics recording
    function process:record_metric(metric_type, value)
        return M.metrics.collect(self.name, metric_type, value)
    end

    -- Update tuning parameters
    function process:update_tuning()
        self._tuning_params = M.tuning.get_params(self.name)
        return true
    end

    return process
end

return M
