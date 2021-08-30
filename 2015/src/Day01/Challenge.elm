module Day01.Challenge exposing ( part1, part2 )

import Day01.Input exposing (..)
import Array exposing ( Array )

part1 : String -> String
part1 input = 
  String.split "" input
  |> List.foldl 
  (\ch acc -> 
    if ch == "(" then
      acc + 1
    else
      acc - 1
  ) 0
  |> String.fromInt


goToBasement : Int -> Int -> Array String -> Int
goToBasement floor index array =
  let
    nextIndex = index + 1

    currentFloor = 
      case Array.get index array of
        Just s ->
          if s == "(" then
            floor + 1
          else
            floor - 1
        Nothing ->
          floor - 1

  in
  if currentFloor < 0 then
    nextIndex
  else 
    goToBasement currentFloor nextIndex array


part2 : String -> String
part2 input = 
  String.split "" input
  |> Array.fromList
  |> goToBasement 0 0
  |> String.fromInt