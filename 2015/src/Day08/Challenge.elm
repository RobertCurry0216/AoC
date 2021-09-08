module Day08.Challenge exposing (..)

import Regex


escapedSymbol : Regex.Regex
escapedSymbol = 
  Maybe.withDefault Regex.never
  <| Regex.fromString "\\\\[\\\"\\\\]"


escapedHex : Regex.Regex
escapedHex =
  Maybe.withDefault Regex.never
  <| Regex.fromString "\\\\x[0-9a-f]{2}"


inMemoryLength : String -> Int
inMemoryLength input =
  let
      total =
        String.length input

      symbols =
        Regex.find escapedSymbol input
        |> List.length

      hexes =
        Regex.find escapedHex input
        |> List.length
        |> (*) 3

  in
  total - symbols - hexes - 2


escapedMemoryLength : String -> Int
escapedMemoryLength input =
  let
      total =
        String.length input

      symbols =
        Regex.find escapedSymbol input
        |> List.length
        |> (*) 2

      hexes =
        Regex.find escapedHex input
        |> List.length

  in
  total + symbols + hexes + 4


totalChars : String -> Int
totalChars input =
  String.length input


part1 : String -> String
part1 input =
  String.lines input
  |> List.filter (\l -> not <| String.isEmpty l)
  |> List.map (\l -> totalChars l - inMemoryLength l)
  |> List.sum
  |> String.fromInt


part2 : String -> String
part2 input =
  String.lines input
  |> List.filter (\l -> not <| String.isEmpty l)
  |> List.map (\l -> escapedMemoryLength l - totalChars l)
  |> List.sum
  |> String.fromInt