module Day15.Challenge exposing (..)

import Parser exposing (..)
import Set


type alias Ingredient =
    { capacity : Int
    , durability : Int
    , flavor : Int
    , texture : Int
    , calories : Int
    }


name : Parser String
name =
  succeed identity
  |= variable
  { start = Char.isAlpha
  , inner = Char.isAlpha
  , reserved = Set.empty
  }


value : String -> Parser Int
value valueName =
  succeed identity
  |. keyword valueName
  |. spaces
  |= oneOf
    [ succeed negate
        |. symbol "-"
        |= int
    , int
    ]


parseIngredient : Parser Ingredient
parseIngredient =
  succeed Ingredient
  |. name
  |. symbol ":"
  |. spaces
  |= value "capacity"
  |. symbol ","
  |. spaces
  |= value "durability"
  |. symbol ","
  |. spaces
  |= value "flavor"
  |. symbol ","
  |. spaces
  |= value "texture"
  |. symbol ","
  |. spaces
  |= value "calories"


ingredientScore : Ingredient -> Int -> Ingredient
ingredientScore i s =
  Ingredient (i.capacity*s) (i.durability*s) (i.flavor*s) (i.texture*s) (i.calories*s)


findHighestScore : Int -> List (List Ingredient) -> Ingredient
findHighestScore spoons scoresLists =
  let
    head =
      List.head scoresLists
      |> Maybe.withDefault []

    rest =
      List.tail scoresLists
  in
  case rest of
    Just r ->
      head
      |> List.take spoons
      |> List.indexedMap
        (\i v ->
          v + (findHighestScore (spoons - i) r)
        )
      |> List.foldl max 0
    Nothing ->
      head
      |> List.take spoons
      |> List.foldl max 0



part1 input =
  input
  |> String.lines
  |> List.filterMap 
    (\l ->
      case run parseIngredient l of
      Ok v -> Just v
      Err _ -> Nothing
    )
  |> List.map
    (\i -> 
      List.range 0 10
      |> List.map ( ingredientScore i )
    )
  |> findHighestScore 10
  |> (\i -> i.capacity * i.durability * i.flavor * i.texture)
  |> String.fromInt


part2 input =
  input