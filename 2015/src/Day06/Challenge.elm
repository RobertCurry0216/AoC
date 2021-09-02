module Day06.Challenge exposing (..)

import Parser exposing (..)


type InstructionMode
  = On
  | Off
  | Toggle


type alias Point =
  { x : Int
  , y : Int
  }


point : Parser Point
point =
  succeed Point
  |= int
  |. symbol ","
  |= int


type alias Instruction =
  { mode : InstructionMode
  , topLeft : Point
  , bottomRight : Point
  }


parseInstructionMode : Parser InstructionMode
parseInstructionMode =
  oneOf
  [ map (\_ -> On) (keyword "turn on")
  , map (\_ -> Off) (keyword "turn off")
  , map (\_ -> Toggle) (keyword "toggle")
  ]

parseInstruction : Parser Instruction
parseInstruction =
  succeed Instruction
  |= parseInstructionMode
  |. spaces
  |= point
  |. spaces
  |. keyword "through"
  |. spaces
  |= point


getInstruction : String -> Result (List DeadEnd) Instruction
getInstruction s =
  run parseInstruction s


type alias Lights =
  List Bool


iToP : Int -> Int -> Point
iToP w idx =
  { x = modBy w idx
  , y = idx // w
  }


doInstruction : Int -> Instruction -> Int -> Bool -> Bool
doInstruction size instruction idx current =
  let
    p = 
      iToP size idx

    isAffected = 
      p.x >= instruction.topLeft.x
      && p.x <= instruction.bottomRight.x
      && p.y >= instruction.topLeft.y
      && p.y <= instruction.bottomRight.y
  in
  if isAffected then
    case instruction.mode of
    On -> 
      True
    Off -> 
      False
    Toggle -> 
      not current
  else
    current


doInstructionOnLights : Int -> Instruction -> Lights -> Lights
doInstructionOnLights size instruction lights = 
  List.indexedMap
  ( doInstruction size instruction )
  lights


part1 : String -> String
part1 input = 
  let
      size = 1000

      startingLights =
        List.repeat (size * size) False
  in
  
  String.split "\n" input
  |> List.map getInstruction
  |> List.filterMap 
    (\i -> case i of
      Ok v -> Just v
      Err _ -> Nothing
    )
  |> List.foldl (doInstructionOnLights size) startingLights
  |> List.filter (\b -> b)
  |> List.length
  |> String.fromInt


type alias ElvishLights =
  List Int


doElvishInstruction : Int -> Instruction -> Int -> Int -> Int
doElvishInstruction size instruction idx current =
  let
    p = 
      iToP size idx

    isAffected = 
      p.x >= instruction.topLeft.x
      && p.x <= instruction.bottomRight.x
      && p.y >= instruction.topLeft.y
      && p.y <= instruction.bottomRight.y
  in
  if isAffected then
    case instruction.mode of
    On -> 
      current + 1
    Off -> 
      Basics.max 0 ( current - 1 )
    Toggle -> 
      current + 2
  else
    current


doElvishInstructionOnLights : Int -> Instruction -> ElvishLights -> ElvishLights
doElvishInstructionOnLights size instruction lights = 
  List.indexedMap
  ( doElvishInstruction size instruction )
  lights


part2 : String -> String
part2 input = 
  let
      size = 1000

      startingLights =
        List.repeat (size * size) 0
  in
  
  String.split "\n" input
  |> List.map getInstruction
  |> List.filterMap 
    (\i -> case i of
      Ok v -> Just v
      Err _ -> Nothing
    )
  |> List.foldl (doElvishInstructionOnLights size) startingLights
  |> List.sum
  |> String.fromInt