(* ::Package:: *)

actual=StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]];

numbers[s_]:=Map[FromDigits,Characters/@StringSplit[s],{2}];

edgeWeighted[numbers_]:=With[{vertices=Table[{x,y},{x,1,Length[numbers]},{y,1,Length[First@numbers]}]},With[{edges=UndirectedEdge@@@Flatten[Join[Table[{{i,j},{i+1,j}},{i,1,Length@numbers-1},{j,1,Length@First@numbers}],Table[{{i,j},{i,j+1}},{i,1,Length@numbers},{j,1,Length@First@numbers-1}]],1]},Graph[Flatten[Table[{x,y},{x,1,Length[numbers]},{y,1,Length[First@numbers]}],1],edges,EdgeWeight->(Extract[numbers,#[[1]]]+Extract[numbers,#[[2]]]&/@edges)
]]]

part1[numbers_]:=Total[Extract[numbers,#]&/@Rest@With[{g=edgeWeighted[numbers]},FindShortestPath[g,{1,1},Dimensions[numbers],Method->"Dijkstra"]]]


part1[numbers[actual]]


With[{original=numbers[actual]},With[{numbers=Mod[Flatten[Map[Flatten,Transpose/@Table[original+i+j,{i,0,4},{j,0,4}],{2}],1],9,1]},part1@numbers]]
