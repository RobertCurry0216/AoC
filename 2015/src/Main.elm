module Main exposing (..)

import Html exposing (text, div)

import Day01.Commands exposing (part1, part2)

main : Html.Html msg
main =
  div [] 
  [ div [] [text "part 1: ", text <| part1]
  , div [] [text "part 2: ", text <| part2]
  ]