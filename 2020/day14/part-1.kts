import java.io.File;
var data = File("input.txt").bufferedReader().readLines()
var mask = data.first().split(" = ").last()
data = data.drop(1)

val memory = mutableMapOf<Long, Long>()

data.forEach {
    if (it.startsWith("mask")) {
        mask = it.split(" = ").last()
    } else {

        val addr = it.substring(it.indexOf('[') + 1, it.lastIndexOf(']')).toLong()
        val value = it.split(" = ").last().toLong()
        var valueAsBin = value.toString(2).padStart(36, '0')

        mask.forEachIndexed { i, c ->
            if (c != 'X') {
                valueAsBin = valueAsBin.replaceRange(i, i + 1, c.toString())
            }
        }

        memory.put(addr, valueAsBin.toLong(2))
    }
}

println(memory.values.sum())