(* ::Package:: *)

parse[s_]:=With[{lines=StringSplit[s,"\n"]},
{lines[[1]],With[{sides=StringSplit[#," -> "]},Characters[First@sides]->{{StringTake[First@sides,{1}],Last@sides},{Last@sides,StringTake[First@sides,{2}]}}]&/@lines[[3;;]]}]

{start,rules}=parse@StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]];


(* ::Text:: *)
(*Part 1*)


step[rules_,chars_]:=Flatten[chars/.rules,1]


With[{l=SortBy[Tally[Join[StringTake[start,{{1},{-1}}],Flatten@Nest[step[rules,#]&,Partition[Characters@start,2,1],10]]],Last]},
Quotient[l[[-1]],2]-Quotient[l[[1]],2]
][[2]]


(* ::Text:: *)
(*Part 2*)


step[rules_,pairs_]:={#[[1,1]],Total[Last/@#]}&/@GatherBy[Flatten[Thread/@(pairs/.rules),1],First]


With[{l=Cases[Total@Flatten[{Times@@@Nest[step[rules,#]&,Tally@Partition[Characters@start,2,1],40],StringTake[start,{{1},{-1}}]}],_Integer,All]},
(Max[l]-Min[l])/2
]
