let LinkType = ./LinkType

let File =
      { Type =
          { src : Text
          , dest : Text
          , linkType : LinkType
          , replaceFiles : Optional Bool
          , replaceDirectories : Optional Bool
          }
      , default =
        { linkType = LinkType.Link
        , replaceFiles = None Bool
        , replaceDirectories = None Bool
        }
      }

in  File