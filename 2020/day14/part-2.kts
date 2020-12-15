import java.io.File;
var data = File("input.txt").bufferedReader().readLines()
var mask = data.first().split(" = ").last()
data = data.drop(1)

val memory = mutableMapOf<Long, Long>()

fun getWriteAddr(addr: String): List<String> {
    if (!addr.contains('X')) return listOf(addr)

    val ret = mutableListOf<String>()

    ret.addAll(getWriteAddr(addr.replaceFirst('X', '0')))
    ret.addAll(getWriteAddr(addr.replaceFirst('X', '1')))

    return ret
}

data.forEach {
    if (it.startsWith("mask")) {
        mask = it.split(" = ").last()
    } else {

        val addr = it.substring(it.indexOf('[') + 1, it.lastIndexOf(']')).toLong()
        val value = it.split(" = ").last().toLong()
        var addrAsBin = addr.toString(2).padStart(36, '0')

        mask.forEachIndexed { i, c ->
            if (c != '0') {
                addrAsBin = addrAsBin.replaceRange(i, i + 1, c.toString())
            }
        }

        getWriteAddr(addrAsBin).forEach {
            memory[it.toLong(2)] = value
        }
    }
}

println(memory.values.sum())