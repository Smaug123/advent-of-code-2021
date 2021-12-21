(* ::Package:: *)

parseLine[s_]:=ToExpression@StringReplace[s,{"["->"{","]"->"}"}]


nums=parseLine/@StringSplit[ReadString[StringJoin[NotebookDirectory[], "/input.txt"]],"\n"];


concat[s1_,s2_]:={s1,s2}


firstOf[s_,position_]:=Switch[Extract[s,position],_List,firstOf[s,Append[position,1]],_Integer,position]
lastOf[s_,position_]:=Switch[Extract[s,position],_List,lastOf[s,Append[position,2]],_Integer,position]


immediatelyBefore[s_,position_]:=With[{chopped=position[[1;;-LengthWhile[Reverse@position,#==1&]-1]]},If[chopped==={},{},lastOf[s,ReplacePart[chopped,-1->1,Heads->False]]]]
immediatelyBefore[s_,{}]:={}
immediatelyAfter[s_,position_]:=With[{chopped=position[[1;;-LengthWhile[Reverse@position,#==2&]-1]]},If[chopped==={},{},firstOf[s,ReplacePart[chopped,-1->2,Heads->False]]]]
immediatelyAfter[s_,{}]:={}


explodePass[s_]:=With[{position=Select[Position[s,_Integer],Length[#]>4&,1]},
If[position==={},{s,False},
With[{toReplace=Most[position[[1]]]},With[{entry=Extract[s,toReplace],before=immediatelyBefore[s,toReplace],after=immediatelyAfter[s,toReplace]},
{ReplacePart[s,{toReplace->0,before->Extract[s,before]+entry[[1]],after->Extract[s,after]+entry[[2]]},Heads->False],True}
]]
]]


splitPass[s_]:=With[{toSplit=FirstPosition[s,_Integer?(#>=10&)]},
If[toSplit===Missing["NotFound"],{s,False},
With[{extract=Extract[s,toSplit]},
{ReplacePart[s,toSplit->{Floor[extract/2],Ceiling[extract/2]}], True}
]]
]


reduce[s_]:=First@NestWhile[
With[{exploded=explodePass[#[[1]]]},
If[exploded[[2]],exploded,splitPass[#[[1]]]]
]&,
{s,True},#[[2]]&]


magnitude[s_List]:=3 magnitude[s[[1]]]+2magnitude[s[[2]]]
magnitude[s_Integer]:=s


(* ::Text:: *)
(*Part 1*)


magnitude@Fold[reduce@*concat,nums[[1]],nums[[2;;]]]//AbsoluteTiming


(* ::Text:: *)
(*Part 2*)


Max[magnitude[reduce[concat@@#]]&/@With[{s=Subsets[nums,{2}]},Join[s,Reverse@s]]]//AbsoluteTiming
