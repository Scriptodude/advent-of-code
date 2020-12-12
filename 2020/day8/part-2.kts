import java.io.File;
import java.util.*

val lines = File("input.txt").bufferedReader().readLines().map { it.split(" ") };

data class State(val acc: Int, val isp: Int)

val states = mutableListOf<State>()

var acc = 0
var isp = 0
var alreadyVisited = mutableSetOf<Int>()
var previousRollBacks = mutableSetOf<Int>()

while(true) {
    if (isp >= lines.size) break

    var instr = lines[isp][0]
    var value = lines[isp][1].toInt()

    if (alreadyVisited.contains(isp)) {
        alreadyVisited.remove(isp)
        previousRollBacks.add(isp)
        do {
            val state = states.last()
            states.removeLast()

            acc = state.acc;
            isp = state.isp;
        } while(previousRollBacks.contains(isp))

        instr = lines[isp][0]
        if (instr == "nop") {
            instr = "jmp"
        } else if (instr == "jmp") {
            instr = "nop"
        }
    }

    alreadyVisited.add(isp)
    when(instr) {
        "acc" -> acc += value
        "jmp" -> isp += value - 1
        else -> {}
    }

    isp += 1
    states.add(State(acc, isp))
}

println(acc)