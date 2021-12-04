(* ::Package:: *)

nums=Import[StringJoin[NotebookDirectory[], "/input.txt"],"Table"];

part1 = 1##&@@Tr[1##&@@@(#/."up"->-"down")]&;

part2=1##&@@Rest@Fold[Function[{state,elt},Switch[elt[[1]],"forward",state+{0,elt[[2]],state[[1]] elt[[2]]},"down",MapAt[elt[[2]]+#&,state,1],"up",MapAt[#-elt[[2]]&,state,1]]],{0,0,0},#]&;
