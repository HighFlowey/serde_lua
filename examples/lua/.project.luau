return {
    name = "Sisyphus Simulator",
    globIgnorePaths = {
        "**/*.spec.lua"
    },
    tree = {
      ["$className"] = "DataModel",
  
      HttpService = {
        ["$className"] = "HttpService",
        ["$properties"] = {
            HttpEnabled = true
        }
      },
  
      ReplicatedStorage = {
        ["$className"] = "ReplicatedStorage",
        ["$path"] = "src/ReplicatedStorage"
      },
  
      StarterPlayer = {
        ["$className"] = "StarterPlayer",
  
        StarterPlayerScripts = {
          ["$className"] = "StarterPlayerScripts",
          ["$path"] = "src/StarterPlayerScripts"
        }
      },
  
      Workspace = {
        ["$className"] = "Workspace",
        ["$properties"] = {
          Gravity = 67.3
        },
  
        Terrain = {
          ["$path"] = "Terrain.rbxm"
        }
      }
    }
  }
