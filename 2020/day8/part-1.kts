import java.io.File;
val lines = File("input.txt").bufferedReader().readLines().map { it.split(" ") };

var acc = 0
var isp = 0
var alreadyVisited = mutableSetOf<Int>()


while(true) {
    val instr = lines[isp][0]
    val value = lines[isp][1].toInt()

    if (alreadyVisited.contains(isp)) break
    alreadyVisited.add(isp)

    when(instr) {
        "acc" -> acc += value
        "jmp" -> isp += value - 1
        else -> {}
    }

    isp += 1
}

println(acc)