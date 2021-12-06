(* ::Package:: *)

nums=FromDigits/@StringSplit[StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]],","];

Clear[f];
f[n_,days_]:=f[n,days]=If[days>n,f[0,days-n],1]
f[0,days_]:=f[0,days]=f[6,days-1]+f[8,days-1]
f[k_,0]:=1


f[#,80]&/@nums//Total


f[#,256]&/@nums//Total
