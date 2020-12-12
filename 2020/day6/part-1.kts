import java.io.File;
var lines = File("input.txt").bufferedReader().readLines();

fun getNextGroup(): List<String> {
    val ret = lines.takeWhile { l -> !l.isBlank() }
    lines = lines.dropWhile { l -> !l.isBlank() }
    lines = lines.drop(1);

    return ret
}

var total = 0

while (true) {
    val group = getNextGroup()
    if (group.isEmpty()) break

    var count = 0

    (97..122).forEach {
        if(group.any { answers -> answers.contains(it.toChar()) }) {
            count += 1
        }
    }

    total += count
}

println(total)