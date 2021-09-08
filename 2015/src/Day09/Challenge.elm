module Day09.Challenge exposing (..)

import Parser exposing (..)
import Set
import Dict exposing (Dict)
import Set exposing (Set)


type alias Path =
  { from : String
  , to : String
  , distance : Int
  }


type alias PathMap = 
  Dict String (Dict String Int)


name : Parser String
name =
  succeed identity
  |= variable 
  { start = Char.isAlpha
  , inner = Char.isAlpha
  , reserved = Set.empty
  }


route : Parser Path
route =
  succeed Path
  |= name
  |. spaces
  |. keyword "to"
  |. spaces
  |= name
  |. spaces
  |. keyword "="
  |. spaces
  |= int


addToMap : Path -> PathMap -> PathMap
addToMap path map =
  let
      fromMap =
        Dict.get path.from map
        |> Maybe.withDefault Dict.empty
        |> Dict.insert path.to path.distance

      toMap = 
        Dict.get path.to map
        |> Maybe.withDefault Dict.empty
        |> Dict.insert path.from path.distance
  in
  Dict.insert path.from fromMap map
  |> Dict.insert path.to toMap


getPathLengths : String -> Set String -> PathMap -> List Int
getPathLengths location visited map =
  let
    branches =
      Dict.get location map
      |> Maybe.withDefault (Dict.empty)

    toVisit =
      Dict.get location map
      |> Maybe.withDefault Dict.empty
      |> Dict.keys
      |> Set.fromList
      |> \s -> Set.diff s visited
      |> Set.toList

    updatedVisited =
      Set.insert location visited
  in
  if List.length toVisit == 0 then
    []
  else
    toVisit
    |> List.map (\b ->
      Dict.get b branches
      |> Maybe.withDefault 0
    )


part1 input = 
  String.lines input
  |> List.filterMap (\v ->
    case run route v of
    Ok value -> Just value
    Err _ -> Nothing
  )
  |> List.foldl addToMap Dict.empty
  |> Dict.map (\k v ->
    
  )
