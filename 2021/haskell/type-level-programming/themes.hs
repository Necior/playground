module Color where

import Data.Word

data PolybarColorScheme = PolybarColorScheme
  { focusedWorkspaceText :: RGB,
    focusedWorkspaceBackground :: RGB,
    visibleWorkspaceText :: RGB,
    visibleWorkspaceBackground :: RGB
  }

data RGB = RGB
  { rgbRed :: Word8,
    rgbGreen :: Word8,
    rgbBlue :: Word8
  }
  deriving (Eq, Show)

polybarColorScheme =
  PolybarColorScheme
    { focusedWorkspaceText = RGB 0xdd 0xa0 0xdd,
      focusedWorkspaceBackground = RGB 0x2a 0x20 0x35,
      visibleWorkspaceText = RGB 0xdd 0xa0 0xdd,
      visibleWorkspaceBackground = RGB 0x2a 0x20 0x35
    }
