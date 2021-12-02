ints = ReadList[StringJoin[NotebookDirectory[], "/input.txt"], "Number"];

part1 = Tr@Clip[Differences@#, {0, 1}] &;

part2 = Tr@Boole[Less @@@ Partition[#, 4, 1][[All, {1, 4}]]] &;
