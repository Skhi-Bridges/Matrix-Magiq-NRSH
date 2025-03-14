local ffi = require('ffi')
local M = {}

-- Windows API definitions
ffi.cdef[[
    typedef unsigned long DWORD;
    typedef int BOOL;
    typedef const char* LPCSTR;
    typedef char* LPSTR;
    
    DWORD GetLogicalDrives(void);
    BOOL GetDiskFreeSpaceA(
        LPCSTR lpRootPathName,
        DWORD* lpSectorsPerCluster,
        DWORD* lpBytesPerSector,
        DWORD* lpNumberOfFreeClusters,
        DWORD* lpTotalNumberOfClusters
    );
]]

local kernel32 = ffi.load('kernel32')

function M.get_available_drives()
    local drives = {}
    local bitmask = kernel32.GetLogicalDrives()
    
    for i = 0, 25 do  -- A to Z
        if bit.band(bitmask, bit.lshift(1, i)) ~= 0 then
            table.insert(drives, string.char(65 + i) .. ":")  -- ASCII 65 = 'A'
        end
    end
    
    return drives
end

function M.get_drive_info(drive)
    local sectors = ffi.new("DWORD[1]")
    local bytes = ffi.new("DWORD[1]")
    local free_clusters = ffi.new("DWORD[1]")
    local total_clusters = ffi.new("DWORD[1]")
    
    local success = kernel32.GetDiskFreeSpaceA(
        drive .. "\\",
        sectors,
        bytes,
        free_clusters,
        total_clusters
    )
    
    if success == 0 then
        return nil
    end
    
    local total_bytes = total_clusters[0] * sectors[0] * bytes[0]
    local free_bytes = free_clusters[0] * sectors[0] * bytes[0]
    
    return {
        drive = drive,
        total_gb = total_bytes / (1024^3),
        free_gb = free_bytes / (1024^3)
    }
end

return M
