#!/usr/bin/env swipl

parse_range(Token, Range) :-
    split_string(Token, "-", "", [StartStr, EndStr]),
    number_string(Start, StartStr),
    number_string(End, EndStr),
    numlist(Start, End, Range).

even_digit_count(N) :-
    number_codes(N, Codes),
    length(Codes, Len),
    Len mod 2 =:= 0.

invalid_id(N) :-
    number_string(N, Str),
    string_length(Str, Len),
    Half is Len // 2,
    sub_string(Str, 0, Half, Half, Left),
    sub_string(Str, Half, Half, 0, Right),
    Left == Right.

% --- part 1: sum invalid IDs in range ---

sum_invalid_in_range(Token, Sum) :-
    parse_range(Token, Range),
    include(even_digit_count, Range, EvenDigitNums),
    include(invalid_id, EvenDigitNums, InvalidIds),
    sum_list(InvalidIds, Sum).

% --- end part 1 ---

% --- part 2: sum invalid IDs in range V2 ---

sum_invalid_in_range_v2(Token, Sum) :-
    parse_range(Token, Range),
    maplist(number_string, Range, RangeStrs),
    include(even_digit_count, Range, EvenDigitNums),
    include(invalid_id, EvenDigitNums, InvalidIds),
    sum_list(InvalidIds, Sum).

% --- end part 2 ---

run(Input) :-
    split_string(Input, ",", "", Tokens),
    maplist(sum_invalid_in_range, Tokens, Part1Sums),
    maplist(sum_invalid_in_range_v2, Tokens, Part2Sums),
    sum_list(Part1Sums, Part1),
    sum_list(Part2Sums, Part2),
    format("Day 2 Part 1: ~w~n", [Part1]),
    format("Day 2 Part 2: ~w~n", [Part2]).

run :-
    read_file_to_string("../input/day02.txt", Input, []),
    run(Input).
