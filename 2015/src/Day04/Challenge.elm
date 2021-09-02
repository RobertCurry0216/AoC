module Day04.Challenge exposing (..)

import MD5 exposing (hex)

--Brute force AF
hashStartsWith : String -> String -> Int -> String
hashStartsWith start pre int =
  let
    hash =
      hex <| pre ++ String.fromInt int
  in
  if String.startsWith start hash then
    String.fromInt int
  else
    hashStartsWith start pre (int + 1)


part1 : String -> String
part1 input =
  hashStartsWith "00000" input 0


part2 : String -> String
part2 input =
  hashStartsWith "000000" input 0