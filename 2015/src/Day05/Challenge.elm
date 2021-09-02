module Day05.Challenge exposing (..)
import Regex
import Parser exposing (..)


has3Vowels : String -> Bool
has3Vowels s =
  String.split "" s
  |> List.filter (\c -> String.contains c "aeiou" )
  |> List.length
  |> (<=) 3


hasDoubleLetter : String -> Bool
hasDoubleLetter s =
  -- This is a very dumb way to do this lol
  containsOneOf [ "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii", "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr", "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz" ] s


containsOneOf : List String -> String -> Bool
containsOneOf list s =
  let
    inString sub =
      String.contains sub s
  in
  List.any inString list


isNice : String -> Bool
isNice s =
  has3Vowels s 
  && hasDoubleLetter s 
  && not (containsOneOf ["ab", "cd", "pq", "xy"] s)
  


part1 : String -> String
part1 input =
  String.split "\n" input
  |> List.filter isNice
  |> List.length
  |> String.fromInt


aba : Regex.Regex
aba = 
  Maybe.withDefault Regex.never
  <| Regex.fromString "(.).\\1"


xyxy : Regex.Regex
xyxy = 
  Maybe.withDefault Regex.never
  <| Regex.fromString "(..).*\\1"


isExtraNice : String -> Bool
isExtraNice s =
  Regex.contains aba s
  && Regex.contains xyxy s



part2 : String -> String
part2 input =
  String.split "\n" input
  |> List.filter isExtraNice
  |> List.length
  |> String.fromInt


doubleLetter : String -> Parser ()
doubleLetter c =
  succeed ()
    |. symbol c
    |. symbol c


anyDoubleLetter : Parser ()
anyDoubleLetter = 
  oneOf 
  [ doubleLetter "a"
  , doubleLetter "b"
  , doubleLetter "c"
  , doubleLetter "d"
  , doubleLetter "e"
  , doubleLetter "f"
  , doubleLetter "g"
  , doubleLetter "h"
  , doubleLetter "i"
  , doubleLetter "j"
  , doubleLetter "k"
  , doubleLetter "l"
  , doubleLetter "m"
  , doubleLetter "n"
  , doubleLetter "o"
  , doubleLetter "p"
  , doubleLetter "q"
  , doubleLetter "r"
  , doubleLetter "s"
  , doubleLetter "t"
  , doubleLetter "u"
  , doubleLetter "v"
  , doubleLetter "w"
  , doubleLetter "x"
  , doubleLetter "y"
  , doubleLetter "z"
  ]