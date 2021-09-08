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


getPathLengths : PathMap -> Set String -> String -> List Int
getPathLengths map visited location =
  let
    branch =
      Dict.get location map
      |> Maybe.withDefault Dict.empty

    toVisit =
      branch
      |> Dict.keys
      |> Set.fromList
      |> \s -> Set.diff s visited
      |> Set.toList

    updatedVisited =
      Set.insert location visited
  in
  if List.isEmpty toVisit then
    [0]
  else
    toVisit
    |> List.map (\l ->
      getPathLengths map updatedVisited l
      |> List.map (\dist -> 
        Dict.get l branch
        |> Maybe.withDefault 0
        |> (+) dist
      )
    )
    |> List.concat



getAllPathLengths : PathMap -> List Int
getAllPathLengths map =
  Dict.keys map
  |> List.map (getPathLengths map Set.empty)
  |> List.concat


part1 : String -> String
part1 input = 
  String.lines input
  |> List.filterMap (\v ->
    case run route v of
    Ok value -> Just value
    Err _ -> Nothing
  )
  |> List.foldl addToMap Dict.empty
  |> getAllPathLengths
  |> List.minimum
  |> \v ->
    case v of
    Just i -> String.fromInt i
    Nothing -> "no value"


part2 : String -> String
part2 input = 
  String.lines input
  |> List.filterMap (\v ->
    case run route v of
    Ok value -> Just value
    Err _ -> Nothing
  )
  |> List.foldl addToMap Dict.empty
  |> getAllPathLengths
  |> List.maximum
  |> \v ->
    case v of
    Just i -> String.fromInt i
    Nothing -> "no value"