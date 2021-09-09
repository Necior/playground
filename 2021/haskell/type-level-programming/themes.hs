module Color where

import qualified Data.Map.Strict as Map
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

newtype ThemeInstance = ThemeInstance {getThemeInstance :: Map.Map String RGB}

myTheme =
  ThemeInstance . Map.fromList $
    [ ("foreground", RGB 0x3a 0x20 0x35),
      ("background", RGB 0xdd 0xa0 0xdd)
    ]

polybarColorScheme :: ThemeInstance -> Maybe PolybarColorScheme
polybarColorScheme (ThemeInstance theme) =
  PolybarColorScheme
    <$> Map.lookup "foreground" theme
    <*> Map.lookup "background" theme
    <*> Map.lookup "foreground" theme
    <*> Map.lookup "background" theme
