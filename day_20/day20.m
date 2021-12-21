(* ::Package:: *)

{rule,grid}=With[{lines=StringSplit[StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]],"\n"]},
{Boole[#==="#"]&/@Characters[First@lines],Map[Boole[#==="#"]&,Characters/@lines[[3;;]],{2}]}];


step[grid_,rule_,n_]:=CellularAutomaton[{FromDigits[Reverse@rule,2],{2,{{256,128,64},{32,16,8},{4,2,1}}},{1,1}},
{grid,0},{{n},All}]


part1[grid_,rule_]:=Tr@Flatten@step[grid,rule,2]


part2[grid_,rule_]:=Tr@Flatten@step[grid,rule,50]


part1[grid,rule]


part2[grid,rule]
