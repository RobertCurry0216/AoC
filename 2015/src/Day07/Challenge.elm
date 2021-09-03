module Day07.Challenge exposing (..)

import Bitwise exposing (and, or, shiftLeftBy, shiftRightBy, complement)
import Parser exposing (..)
import Set



type OpCode
  = Not
  | And
  | Or
  | LShift
  | RShift
  | NoOp

type alias Op =
  { code : OpCode
  , value1 : Int
  , value2 : Int
  , out : String
  }


parseNoOp : Parser Op
parseNoOp =
  succeed (\i s -> Op NoOp i 0 s)
    |= int
    |. spaces
    |. keyword "->"
    |. spaces
    |= variable 
      { start = Char.isAlpha
      , inner = Char.isAlpha
      , reserved = Set.empty
      }



part1 : String -> String
part1 input =
  input


part2 : String -> String
part2 input =
  input