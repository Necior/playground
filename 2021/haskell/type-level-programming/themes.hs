module Color where

data PolybarColorScheme = PolybarColorScheme
  { focusedWorkspaceText :: String,
    focusedWorkspaceBackground :: String,
    visibleWorkspaceText :: String,
    visibleWorkspaceBackground :: String
  }

polybarColorScheme =
  PolybarColorScheme
    { focusedWorkspaceText = "#dda0dd",
      focusedWorkspaceBackground = "#2a2035",
      visibleWorkspaceText = "#dda0dd",
      visibleWorkspaceBackground = "#2a2035"
    }
