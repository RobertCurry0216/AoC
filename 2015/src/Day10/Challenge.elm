module Day10.Challenge exposing (..)

import Regex


repeatedNums : Regex.Regex
repeatedNums =
  Maybe.withDefault Regex.never
  <| Regex.fromString "(\\d)(\\1)*"


matchToTuple : Regex.Match -> (Int, Int)
matchToTuple match =
  let
    repeat =
      match.match
      |> String.length

    value =
      match.submatches
      |> List.filterMap identity
      |> List.head
      |> Maybe.withDefault ""
      |> String.toInt
      |> Maybe.withDefault -1
      
  in
  (repeat, value)


lookAndSay : String -> String
lookAndSay input =
  input
  |> Regex.find repeatedNums
  |> List.map matchToTuple
  |> List.foldl 
    (\(r,v) acc -> acc ++ (String.fromInt r) ++ (String.fromInt v))
    ""


part1 : String -> String
part1 input =
  List.repeat 40 ()
  |> List.foldl
    (\_ acc -> lookAndSay acc )
    input
  |> String.length
  |> String.fromInt


part2 : String -> String
part2 input =
  List.repeat 50 ()
  |> List.foldl
    (\_ acc -> lookAndSay acc )
    input
  |> String.length
  |> String.fromInt