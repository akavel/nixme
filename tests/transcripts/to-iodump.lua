local function printf(fmt, ...)
    io.write(fmt:format(...))
end

local dump = false
for line in io.lines() do
    local read = line:match '^%d+ +read%(0,'
    local write = line:match '^%d+ +write%(1,'
    if read or write then
        local n = line:match '%d+$'
        printf("\n# %s\n%s # 0s  %d bytes\n",
            line,
            read and '->' or '<-',
            n)
        dump = true
    elseif dump and line:match '^ |' then
        print((line:gsub('^ | [0-9a-f]+  ', ''):gsub('  ', ' ', 1):gsub('  ', '   # ', 1)))
    else
        dump = false
    end
end
