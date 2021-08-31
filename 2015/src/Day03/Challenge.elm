module Day03.Challenge exposing (..)


import Set exposing (Set)
import Debug exposing (..)


type alias Position = (Int, Int)


gotoNextHouse : Position -> Char -> Result String Position
gotoNextHouse ( x, y ) dir =
  case dir of
  '>' ->
    Ok ( x + 1, y )
  '<' ->
    Ok ( x - 1, y )
  '^' ->
    Ok ( x, y + 1 )
  'v' ->
    Ok ( x, y - 1 )
  _ ->
    Err 
    <| Debug.log "error:" 
    <| "unknown: " ++ String.fromChar dir


visitAllHouses : Set Position -> Position -> List Char -> Set Position
visitAllHouses visited pos map =
  let
    newPos =
      List.head map
      |> Maybe.withDefault '_'
      |> gotoNextHouse pos
      |> \p ->
      case p of
        Ok value -> value
        Err _ -> pos

    newMap =
      List.drop 1 map

    newVisited =
      Set.insert newPos visited
  in
  if List.isEmpty newMap then
    newVisited
  else
    visitAllHouses newVisited newPos newMap


everySecond : List a -> List a
everySecond list =
  let
      head =
        List.head list
  in
  case head of
  Just v ->
    v :: ( everySecond <| List.drop 2 list )
  Nothing ->
    list


part1 : String -> String
part1 input = 
  String.toList input
  |> visitAllHouses ( Set.singleton <| Tuple.pair 0 0 ) ( 0, 0 )
  |> Set.size
  |> String.fromInt


part2 : String -> String
part2 input = 
  let
    map =
      String.toList input
  
    santa =
      visitAllHouses 
        ( Set.singleton <| Tuple.pair 0 0 )
        ( 0, 0 )
        <| everySecond map

    roboSanta =
      visitAllHouses 
        ( Set.singleton <| Tuple.pair 0 0 )
        ( 0, 0 )
        <| everySecond <| List.drop 1 map

  in
  Set.union santa roboSanta
  |> Set.size
  |> String.fromInt