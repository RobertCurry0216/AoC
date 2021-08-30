module Main exposing (..)

import Html exposing (text, div)

import Day01.Challenge exposing (part1, part2)
import Day01.Input exposing (input)

main : Html.Html msg
main =
  div [] 
  [ div [] [text "part 1: ", text <| part1 input]
  , div [] [text "part 2: ", text <| part2 input]
  ]