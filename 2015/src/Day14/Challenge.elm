module Day14.Challenge exposing (..)

import Parser exposing (..)
import Set
import Dict


type alias Reindeer =
    { name : String
    , speed : Int
    , sprint : Int
    , rest : Int
    }


name : Parser String
name =
  succeed identity
  |= variable
  { start = Char.isAlpha
  , inner = Char.isAlpha
  , reserved = Set.empty
  }


parseReindeer : Parser Reindeer
parseReindeer =
  succeed Reindeer
  |= name
  |. spaces
  |. keyword "can fly"
  |. spaces
  |= int
  |. spaces
  |. keyword "km/s for"
  |. spaces
  |= int
  |. spaces
  |. keyword "seconds, but then must rest for"
  |. spaces
  |= int


distanceAfter : Int -> Reindeer -> Float
distanceAfter seconds reindeer =
  let
    cycle =
      (reindeer.sprint + reindeer.rest)

    distancePerSprint =
      reindeer.speed * reindeer.sprint

    sprints = 
      seconds // cycle

    extraTime =
      Basics.modBy cycle seconds
      |> (\t -> 
        if t >= reindeer.sprint then
          1.0
        else
          toFloat t / toFloat reindeer.sprint
      )
  in
  (toFloat distancePerSprint * toFloat sprints) + ((toFloat distancePerSprint) * extraTime)


part1 : String -> String
part1 input =
  input
  |> String.lines
  |> List.filterMap (\l ->
    case run parseReindeer l of
    Ok r -> Just r
    Err _ -> Nothing
  )
  |> List.map (distanceAfter 2503)
  |> List.maximum
  |> Maybe.withDefault 0
  |> String.fromFloat


getFurtherest : List (Reindeer, Float) -> Reindeer
getFurtherest list =
  list
  |> List.foldl
    (\(r, d) (br, bd) -> if d > bd then (r, d) else (br, bd))
    (Reindeer "null" 0 0 0, 0)
  |> Tuple.first


part2 : String -> String
part2 input =
  let
    reindeers =
      input
      |> String.lines
      |> List.filterMap (\l ->
        case run parseReindeer l of
        Ok r -> Just r
        Err _ -> Nothing
      ) 
  in
  List.range 1 2503
  |> List.foldl (\time scores ->
    reindeers
    |> List.map (\r -> (r, distanceAfter time r))
    |> getFurtherest
    |> (\r ->
      Dict.get r.name scores
      |> Maybe.withDefault 0
      |> (+) 1
      |> \s -> Dict.insert r.name s scores
    )
  ) Dict.empty
  |> Dict.values
  |> List.maximum
  |> Maybe.withDefault 0
  |> String.fromInt
  