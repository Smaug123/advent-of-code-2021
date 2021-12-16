(* ::Package:: *)

nums=With[{chars=Characters@StringTrim@ReadString[StringJoin[NotebookDirectory[], "/input.txt"]]},Flatten[IntegerDigits[FromDigits[#,16],2,4]&/@chars]];


consumePacket[{}]={};
consumePacket[{v1_,v2_,v3_,t1_,t2_,t3_,otherBits___}]:=With[{packetVersion=Sow@FromDigits[{v1,v2,v3},2],typeId=FromDigits[{t1,t2,t3},2], rest={otherBits}},
Switch[typeId,
4,With[{thisPacket=TakeWhile[Partition[rest,UpTo@5],#[[1]]==1&]},
{FromDigits[Flatten[Join[Rest/@thisPacket,rest[[5 Length@thisPacket+2;;5 Length@thisPacket+5]]]],2],rest[[5 Length@thisPacket+6;;]]}
],

_,
With[{operator=
Switch[typeId,0,Plus,1,Times,2,Min,3,Max,5,Boole@*Greater,6,Boole@*Less,7,Boole@*Equal]
},
Switch[rest[[1]],
0,With[{subPacketLen=FromDigits[rest[[2;;16]],2]},
{operator@@consumePackets[rest[[17;;16+subPacketLen]]],rest[[17+subPacketLen;;]]}
],
1,With[{subPackets=FromDigits[rest[[2;;12]],2]},
With[{result=Nest[With[{c=consumePacket[#[[2]]]},{Join[#[[1]],{c[[1]]}],c[[2]]}]&,{{},rest[[13;;]]},subPackets]},
{operator@@result[[1]],result[[2]]}
]
]
]

]
]]

consumePackets[l_]:=First@NestWhile[With[{c=consumePacket[#[[2]]]},{Join[#[[1]],{c[[1]]}],c[[2]]}]&,{{},l},Not@AllTrue[#[[2]],#==0&]&]


MapAt[Total@*Flatten,Reap@consumePackets[nums],{2}]
