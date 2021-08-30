module Day02.Challenge exposing ( .. )

import Day02.Input exposing (..)
import Parser exposing ( .. )


type alias Area =
    { l : Int
    , w : Int
    , h : Int
    }


areaParser : Parser Area
areaParser =
  succeed Area
  |= int
  |. symbol "x"
  |= int
  |. symbol "x"
  |= int


extractArea : String -> Area
extractArea s =
  let
    parsed =
      run areaParser s
  in
  case parsed of
    Ok area -> area
    Err _ -> Area 0 0 0


getRequiredPaper : Area -> Int
getRequiredPaper area =
  let
    side1 = area.w * area.l
    side2 = area.h * area.w
    side3 = area.l * area.h

    smallest = 
      case List.minimum [side1, side2, side3] of
      Just n -> n
      Nothing -> 0
  in
  2*side1 + 2*side2 + 2*side3 + smallest


getRequiredRibbon : Area -> Int
getRequiredRibbon area =
  let
    sides =
      [area.w, area.l, area.h]

    parimiter =
      List.sort sides
      |> List.take 2
      |> List.sum
      |> (*) 2

    volumn =
      List.foldr (*) 1 sides
  in
  parimiter + volumn



part1 : String -> String
part1 input = 
  String.split "\n" input
  |> List.map (\p -> 
    extractArea p
    |> getRequiredPaper
  )
  |> List.sum
  |> String.fromInt


part2 : String -> String
part2 input = 
  String.split "\n" input
  |> List.map (\p -> 
    extractArea p
    |> getRequiredRibbon
  )
  |> List.sum
  |> String.fromInt