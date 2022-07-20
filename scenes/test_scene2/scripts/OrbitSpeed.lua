evaluate = function(t)
  local result = 5 + t*t*0.07
  if(t>7.0) then
    local curResult = (t - 7.0) * 0.5
    return result - curResult * curResult
  end
  return result
end
