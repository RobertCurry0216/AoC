module Day12.Challenge exposing (..)

import Regex exposing (Regex)


number : Regex
number =
  Maybe.withDefault Regex.never
  <| Regex.fromString "-?\\d+"


red : Regex
red =
  Maybe.withDefault Regex.never
  <| Regex.fromString "({.*?\"red\".*?})"


redInList : Regex
redInList =
  Maybe.withDefault Regex.never
  <| Regex.fromString "(\\[.*?\"red\".*?\\])"


sumNumbers : String -> Int
sumNumbers input =
  input
  |> Regex.find number
  |> List.foldl (\m acc -> 
    case String.toInt m.match of
    Just v -> acc + v
    Nothing -> acc
  ) 0


part1 : String -> String
part1 input = 
  sumNumbers input
  |> String.fromInt


part2 : String -> String
part2 input =
  let
    total =
      sumNumbers input

    totalRed =
      Regex.find red input
      |> List.foldl (\m acc -> 
        if Regex.contains redInList m.match then
          acc
        else
          acc + (sumNumbers m.match)
      ) 0
  in
  total - totalRed
  |> String.fromInt