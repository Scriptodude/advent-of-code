import java.io.File;
val lines = File("input.txt").bufferedReader().readLines().map { it.trim().toInt() }.sorted()

var current = 0
var step3 = 1
var step1 = 0

lines.forEach {
    if (it - current <= 3) {
        when(it - current) {
            1 -> step1++
            3 -> step3++
            else -> {}
        }
    }

    current = it
}

println(step3 * step1)