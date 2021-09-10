module Day13.Challenge exposing (..)


import Parser exposing (..)
import Set


type alias SeatingPair =
  { person : String
  , happiness : Int
  , ajacent : String
  }


name : Parser String
name =
  succeed identity
  |= variable
  { start = Char.isAlpha
  , inner = Char.isAlpha
  , reserved = Set.empty
  }


happinessGain : Parser Int
happinessGain =
  succeed identity
  |. keyword "gain"
  |. spaces
  |= int


happinessLose : Parser Int
happinessLose =
  succeed (\i -> -i)
  |. keyword "lose"
  |. spaces
  |= int



parseSeatingPair : Parser SeatingPair
parseSeatingPair =
  succeed SeatingPair
  |= name
  |. spaces
  |. keyword "would"
  |. spaces
  |= oneOf
    [ happinessGain
    , happinessLose
    ]
  |. spaces
  |. keyword "happiness units by sitting next to"
  |. spaces
  |= name


permutations : List a -> List (List a)
permutations list =
  let
    subList ex =
      List.filter (\v -> v /= ex) list
  in
  if List.length list <= 1 then
    [list]
  else
    list
    |> List.map (\h -> 
      permutations (subList h)
      |> List.map (\t -> h :: t)
    )
    |> List.concat


getTriplets : List a -> List (a, a, a)
getTriplets list =
  let
    extendedList =
      List.append list (List.take 2 list)

    trips l =
      case l of
      (a::b::c::_) -> (a,b,c) :: (trips (List.drop 1 l))
      _ -> []
  in
  trips extendedList


getHappinessScore : List SeatingPair -> List String -> Int
getHappinessScore pairs plan =
  let
    getPairScore person ajacent =
      pairs
      |> List.foldl (\seat curr ->
        if seat.person == person && seat.ajacent == ajacent then
          seat.happiness
        else 
          curr
      ) 0

    getTripletScore (a,b,c) =
      (getPairScore b a) + (getPairScore b c)
  in
  plan
  |> getTriplets
  |> List.map getTripletScore
  |> List.sum


getSeatingPairs : String -> List SeatingPair
getSeatingPairs input =
  input
  |> String.lines
  |> List.filterMap (\s -> 
    case run parseSeatingPair s of
    Ok v -> Just v
    Err _ -> Nothing
  )


part1 input =
  let
    seatingPairs =
      getSeatingPairs input

    people =
      seatingPairs
      |> List.map .person
      |> Set.fromList
      |> Set.toList
  in
  people
  |> permutations
  |> List.map (getHappinessScore seatingPairs)
  |> List.maximum
  |> Maybe.withDefault 0
  |> String.fromInt


part2 input =
  let
    seatingPairs =
      getSeatingPairs input

    people =
      seatingPairs
      |> List.map .person
      |> Set.fromList
      |> Set.toList
      |> (::) "Robert"
  in
  people
  |> permutations
  |> List.map (getHappinessScore seatingPairs)
  |> List.maximum
  |> Maybe.withDefault 0
  |> String.fromInt