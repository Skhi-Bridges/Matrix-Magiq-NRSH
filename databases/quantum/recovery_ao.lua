#!/usr/bin/env luajit

local ffi = require('ffi')
local json = require('cjson')

-- Service Recovery AO
local RecoveryAO = {
    name = "service-recovery-ao",
    state = {
        services = {},
        error_counts = {},
        backoff_times = {}
    },
    config = {
        max_retries = 5,
        initial_backoff = 1,  -- seconds
        max_backoff = 300,    -- 5 minutes
        error_threshold = 3,   -- errors before circuit breaker
        recovery_interval = 5  -- check every 5 seconds
    }
}

-- Circuit breaker states
local CircuitState = {
    CLOSED = "CLOSED",       -- Normal operation
    OPEN = "OPEN",          -- Failing, not accepting requests
    HALF_OPEN = "HALF_OPEN" -- Testing if service recovered
}

-- Initialize circuit breaker for a service
function RecoveryAO:init_circuit(service_name)
    self.state.services[service_name] = {
        state = CircuitState.CLOSED,
        last_error = nil,
        last_success = os.time(),
        consecutive_failures = 0
    }
end

-- Handle 502 Bad Gateway
function RecoveryAO:handle_502(service_name)
    local service = self.state.services[service_name]
    if not service then
        self:init_circuit(service_name)
        service = self.state.services[service_name]
    end

    service.consecutive_failures = service.consecutive_failures + 1
    service.last_error = os.time()

    -- Check if we should open the circuit
    if service.consecutive_failures >= self.config.error_threshold then
        self:open_circuit(service_name)
    end

    -- Calculate exponential backoff
    local backoff = self.config.initial_backoff * (2 ^ service.consecutive_failures)
    backoff = math.min(backoff, self.config.max_backoff)
    self.state.backoff_times[service_name] = backoff

    print(string.format("[%s] Service %s failed with 502. Circuit: %s, Backoff: %ds",
        os.date(), service_name, service.state, backoff))
end

-- Open circuit breaker
function RecoveryAO:open_circuit(service_name)
    local service = self.state.services[service_name]
    service.state = CircuitState.OPEN
    
    -- Schedule half-open test
    local backoff = self.state.backoff_times[service_name] or self.config.initial_backoff
    
    print(string.format("[%s] Opening circuit for %s. Will test in %ds",
        os.date(), service_name, backoff))
end

-- Test service recovery
function RecoveryAO:test_service(service_name)
    local service = self.state.services[service_name]
    if service.state ~= CircuitState.OPEN then return end

    service.state = CircuitState.HALF_OPEN
    print(string.format("[%s] Testing service %s...", os.date(), service_name))

    -- Implement actual service test here
    local success = false -- Replace with actual test
    
    if success then
        self:close_circuit(service_name)
    else
        self:handle_502(service_name) -- Reset backoff and keep circuit open
    end
end

-- Close circuit after recovery
function RecoveryAO:close_circuit(service_name)
    local service = self.state.services[service_name]
    service.state = CircuitState.CLOSED
    service.consecutive_failures = 0
    service.last_success = os.time()
    self.state.backoff_times[service_name] = nil

    print(string.format("[%s] Service %s recovered. Circuit closed.",
        os.date(), service_name))
end

-- Main recovery loop
function RecoveryAO:run()
    print(string.format("[%s] Starting %s...", os.date(), self.name))

    while true do
        -- Check all services
        for service_name, service in pairs(self.state.services) do
            if service.state == CircuitState.OPEN then
                local backoff = self.state.backoff_times[service_name]
                if os.time() - service.last_error >= backoff then
                    self:test_service(service_name)
                end
            end
        end

        -- Wait for next check
        os.execute("sleep " .. self.config.recovery_interval)
    end
end

-- Example usage
RecoveryAO:init_circuit("quantum-db")
RecoveryAO:init_circuit("metrics-service")
RecoveryAO:run()
