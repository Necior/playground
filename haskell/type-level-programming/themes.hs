{-# LANGUAGE GADTs #-}

module Color where

import qualified Data.Map.Strict as Map
import Data.Word
import Text.Printf

data PolybarColorScheme = PolybarColorScheme
  { focusedWorkspaceText :: RGB,
    focusedWorkspaceBackground :: RGB,
    visibleWorkspaceText :: RGB,
    visibleWorkspaceBackground :: RGB
  }

data SomeColor = forall color. IsColor color => SomeColor color

instance Show SomeColor where
  show = show . toRGB

data RGB = RGB
  { rgbRed :: Word8,
    rgbGreen :: Word8,
    rgbBlue :: Word8
  }
  deriving (Eq, Show)

newtype ThemeInstance = ThemeInstance {getThemeInstance :: Map.Map String SomeColor}

class IsColor a where
  toRGB :: a -> RGB

instance IsColor RGB where
  toRGB = id

instance IsColor SomeColor where
  toRGB (SomeColor color) = toRGB color

data AliceBlue = AliceBlue

instance IsColor AliceBlue where
  toRGB = const $ RGB 0xf0 0xf8 0xff

-- many other X11 color could go here

someRGB :: Word8 -> Word8 -> Word8 -> SomeColor
someRGB r g b = SomeColor $ RGB r g b

t =
  ThemeInstance $
    Map.insert
      "foreground"
      (someRGB 255 0 0)
      (Map.singleton "background" (SomeColor AliceBlue))

polybarColorScheme :: ThemeInstance -> Maybe PolybarColorScheme
polybarColorScheme (ThemeInstance theme) =
  PolybarColorScheme
    <$> (toRGB <$> Map.lookup "foreground" theme)
    <*> (toRGB <$> Map.lookup "background" theme)
    <*> (toRGB <$> Map.lookup "foreground" theme)
    <*> (toRGB <$> Map.lookup "background" theme)

toHex :: IsColor a => a -> String
toHex a =
  let (RGB r g b) = toRGB a
   in printf "%02x%02x%02x" r g b
