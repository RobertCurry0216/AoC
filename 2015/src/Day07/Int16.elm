module Day07.Int16 exposing (fromInt, toInt, lShift, rShift, and, or, complement, Int16, zero)

import Bitwise


type alias Int16 =
  List Int


zero : Int16
zero =
  List.repeat 16 0


ones : Int16
ones =
  List.repeat 16 1


fromInt : Int -> Int16
fromInt int =
  if int >= 65535 then
    ones
  else if int <= 0 then
    zero
  else
    [ Bitwise.shiftRightBy 15 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 14 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 13 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 12 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 11 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 10 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 9 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 8 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 7 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 6 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 5 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 4 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 3 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 2 int |> Bitwise.and 1
    , Bitwise.shiftRightBy 1 int |> Bitwise.and 1
    , Bitwise.and 1 int
    ]


toInt : Int16 -> Int
toInt int =
  List.foldl (\a b -> Bitwise.shiftLeftBy 1 b |> Bitwise.or a) 0 int


lShift : Int -> Int16 -> Int16
lShift shift int =
  let
    head = 
      List.drop shift int

    tail =
      List.take shift zero
  in
  List.append head tail
  

rShift : Int -> Int16 -> Int16
rShift shift int =
  let
    tail = 
      List.take (16 - shift) int

    head =
      List.take shift zero
  in
  List.append head tail


and : Int16 -> Int16 -> Int16
and int1 int2 =
  List.map2 (\a b -> Bitwise.and a b) int1 int2


or : Int16 -> Int16 -> Int16
or int1 int2 =
  List.map2 (\a b -> Bitwise.or a b) int1 int2


complement : Int16 -> Int16
complement int =
  let
      bitwise_not a =
        if a == 1 then
          0
        else
          1
  in
  List.map bitwise_not int