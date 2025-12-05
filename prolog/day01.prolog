#!/usr/bin/env swipl

% ./day01.prolog
% and then run: run.

split_lines(String, Lines) :-
    split_string(String, "\n", "\n", Lines).

split_line(Line, Turn) :-
    string_chars(Line, Chars),
    chars_to_instructions(Chars, Turn).

chars_to_instructions(['L' | Rest], Turn) :-
    string_chars(String, Rest),
    atom_number(String, N),
    Turn is -N.

chars_to_instructions(['R' | Rest], Turn) :-
    string_chars(String, Rest),
    atom_number(String, Turn).

read_file_to_string(Path, String) :-
    open(Path, read, Stream),
    read_string(Stream, _, String),
    close(Stream).

% --- part 1: get password ---

get_password(Instructions, Dial, Acc) :-
    get_password(Instructions, 50, 0, Dial, Acc).

get_password([Instruction | Rest], Dial, Acc, FinalDial, FinalAcc) :-
    NewDial is (Dial + Instruction) mod 100,
    (NewDial =:= 0 -> NewAcc is Acc + 1 ; NewAcc = Acc),
    get_password(Rest, NewDial, NewAcc, FinalDial, FinalAcc).

get_password([], Dial, Acc, FinalDial, FinalAcc) :-
    FinalDial = Dial,
    FinalAcc = Acc.

% --- end part 1 ---

% --- part 2: get password v2 ---

get_password_v2(Instructions, Dial, Acc) :-
    get_password_v2(Instructions, 50, 0, Dial, Acc).

get_password_v2([Instruction | Rest], Dial, Acc, FinalDial, FinalAcc) :-
    (Instruction >= 0 -> 
        NewAcc is Acc + (Dial + Instruction) // 100
        ;
        RevDial is (100 - Dial) rem 100,
        NewAcc is Acc + (RevDial - Instruction) // 100
    ),
    NewDial is (Dial + Instruction) mod 100,
    get_password_v2(Rest, NewDial, NewAcc, FinalDial, FinalAcc).

get_password_v2([], Dial, Acc, FinalDial, FinalAcc) :-
    FinalDial = Dial,
    FinalAcc = Acc.

% --- end part 2 ---

run(Input) :-
    split_lines(Input, Lines),
    maplist(split_line, Lines, Instructions),

    get_password(Instructions, _, Part1),
    get_password_v2(Instructions, _, Part2),

    format("Day 1 Part 1: ~w~n", [Part1]),
    format("Day 1 Part 2: ~w~n", [Part2]).

run() :-
    read_file_to_string("../input/day01.txt", Input),
    run(Input).
