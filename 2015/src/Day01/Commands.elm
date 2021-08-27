module Day01.Commands exposing (..)

import File exposing (File)
import Task
import Html exposing (..)

type InputMsg
  = InputLoaded String
  | InputPending

read : File -> Cmd InputMsg
read file =
  Task.perform InputLoaded (File.toString file)

part1 : String
part1 = 
  let
    input = "(())"
    output = input 
      |> String.foldl 
      <| (/c v = 1)
      
  in
    output


part2 : String
part2 = "2"