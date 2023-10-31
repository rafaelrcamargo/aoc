module aoc

cwd = ""

function cd(line::String)
  path = replace(line, "\$ cd " => "")
  return normpath(cwd * path * "/")
end

function file(line::String)
  parse(UInt, split(line, " ")[1])
end

function part1(lines::Vector{String})
  tree::Dict{String,UInt} = Dict("/" => 0)

  for line in lines
    if startswith(line, "\$ ls") || startswith(line, "dir ")
      # println("List or Dir: ", line)
      continue
    elseif startswith(line, "\$ cd")
      # println("Change: ", line)
      global cwd = cd(line)
    else
      # println("File: ", line)
      tree[cwd] = get(tree, cwd, 0) + file(line)
    end
  end

  sorted = reverse(sort(collect(tree), by=x -> length(split(x[1], "/"))))
  println(sorted)

  for (key, _) in sorted
    for (dir, _) in sorted
      if startswith(dir, key) && dir != key
        # println(dir * " => " * key)
        tree[key] = tree[key] + tree[dir]
      end
    end
  end

  # println(tree)

  tree |> values |> collect |> filter(size -> size <= 100000) |> sum
end

end

open("./assets/input.txt", "r") do file
  println(aoc.part1(readlines(file)))
end
