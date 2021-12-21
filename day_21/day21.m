(* ::Package:: *)

pos=FromDigits@Last@StringSplit[#,": "]&/@StringSplit[StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]],"\n"]


(* ::Text:: *)
(*Part 2*)


Clear[roll];
roll[pos1_,pos2_,score1_,score2_]:=roll[pos1,pos2,score1,score2]=Function[{dieRolls},
With[{finalPos=Mod[pos1+Total[dieRolls],10,1]},
With[{score=score1+finalPos},
If[score>=21,{1,0},Reverse@roll[pos2,finalPos,score2,score]]
]
]
]/@Tuples[{1,2,3},{3}]//Total


part2[{player1_,player2_}]:=Max@roll[player1,player2,0,0]


part2[pos]//Max
