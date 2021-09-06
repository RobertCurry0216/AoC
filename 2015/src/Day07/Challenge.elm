module Day07.Challenge exposing (..)

import Parser exposing (..)
import Set
import Dict exposing ( Dict )
import Day07.Int16 exposing (..)



type OpCode
  = Not
  | And
  | Or
  | LShift
  | RShift
  | Identity
  | NoOp


type Wire
  = Value Int16
  | Pointer String
  | NoValue


type alias Memo = Dict String Int16


type alias WireMap = Dict String Op


type alias Op =
  { in1 : Wire
  , code : OpCode
  , in2 : Wire
  , out : String
  }


noop : Op
noop =
  Op NoValue NoOp NoValue ""


identifier : Parser Wire
identifier = 
  succeed identity
    |= variable 
      { start = Char.isAlphaNum
      , inner = Char.isAlphaNum
      , reserved = Set.empty
      }
  |> andThen (\s ->
    case String.toInt s of
      Just v -> 
        succeed (Value (fromInt v))
      Nothing ->
        succeed (Pointer s)
  )
    


parseIdentity : Parser Op
parseIdentity =
  succeed (\in1 out -> Op in1 Identity NoValue out)
    |= identifier
    |. spaces
    |. keyword "->"
    |. spaces
    |= variable 
      { start = Char.isAlphaNum
      , inner = Char.isAlphaNum
      , reserved = Set.empty
      }


parseNot : Parser Op
parseNot =
  succeed (\ in1 out -> Op in1 Not NoValue out)
    |. keyword "NOT"
    |. spaces
    |= identifier
    |. spaces
    |. keyword "->"
    |. spaces
    |= variable 
      { start = Char.isAlphaNum
      , inner = Char.isAlphaNum
      , reserved = Set.empty
      }


parseOpCode : Parser Op
parseOpCode =
  succeed Op
  |= identifier
  |. spaces
  |= oneOf
    [ map (\_ -> And) (keyword "AND")
    , map (\_ -> Or) (keyword "OR")
    , map (\_ -> RShift) (keyword "RSHIFT")
    , map (\_ -> LShift) (keyword "LSHIFT")
    ]
  |. spaces
  |= identifier
  |. spaces
  |. keyword "->"
  |. spaces
  |= variable 
      { start = Char.isAlphaNum
      , inner = Char.isAlphaNum
      , reserved = Set.empty
      }


parseOp : Parser Op
parseOp = 
  oneOf
  [ parseNot
  , backtrackable parseOpCode
  , parseIdentity
  ]


getOpCode : String -> Op
getOpCode s =
  case run parseOp s of
    Ok op ->
      op
    Err _ ->
      noop


doSingleOp : OpCode -> Int16 -> Int16 -> Result String Int16
doSingleOp code v1 v2 =
  case code of
    Identity ->
      Ok v1
    And -> 
      Ok <| and v1 v2
    Or ->
      Ok <| or v1 v2
    RShift ->
      Ok <| rShift (toInt v2) v1
    LShift ->
      Ok <| lShift (toInt v2) v1
    Not ->
      Ok <| complement v1
    _ -> 
      Err "No op"


mapSignal : String -> Op -> (Memo, WireMap, Int) -> (Memo, WireMap, Int)
mapSignal out op (memo, wires, errs) =
  let 
    value1 =
      case op.in1 of
      Value v ->
        Ok v
      Pointer p ->
        case Dict.get p memo of
          Just v -> Ok v
          Nothing -> Err "Wire not connected"
      NoValue -> 
        Ok zero

    value2 =
      case op.in2 of
      Value v ->
        Ok v
      Pointer p ->
        case Dict.get p memo of
          Just v -> Ok v
          Nothing -> Err "Wire not connected"
      NoValue -> 
        Ok zero

  in
  case (value1, value2) of
  (Ok v1, Ok v2) ->
    doSingleOp op.code v1 v2
    |> \r ->
      case r of
      Ok v -> (Dict.insert out v memo, wires, errs)
      Err _ -> (memo, wires, errs + 1)
  _ ->
    (memo, wires, errs + 1)


mapSignals : Int -> Memo -> WireMap -> Memo
mapSignals errs memo wires =
  if errs == 0 then
    memo
  else
    Dict.foldl mapSignal (memo, wires, 0) wires
    |> (\(m, w, e) -> mapSignals e m w)


part1 : String -> String
part1 input =
  String.lines input
  |> List.map getOpCode
  |> List.foldl 
    (\op wires ->
      Dict.insert op.out op wires
    ) Dict.empty
  |> mapSignals 1 Dict.empty
  |> Dict.get "a"
  |> \r ->
  case r of
    Just v -> String.fromInt (toInt v)
    Nothing -> "no value"

part2 : String -> String
part2 input =
  String.lines input
  |> List.map getOpCode
  |> List.foldl 
    (\op wires ->
      Dict.insert op.out op wires
    ) Dict.empty
  |> Dict.insert "b" (Op (Value (fromInt 3176)) Identity NoValue "b")
  |> mapSignals 1 Dict.empty
  |> Dict.get "a"
  |> \r ->
  case r of
    Just v -> String.fromInt (toInt v)
    Nothing -> "no value"