import java.io.File;
import java.util.*
import kotlin.collections.HashMap
var lines = File("input.txt").bufferedReader().readLines();

data class Bag (val id: String) {
    var count: Int = 0
}
val NO_BAG = Bag("")

fun getAllBagsOfLine(line: String) : List<Bag> {
    if (line.isBlank()) return mutableListOf();

    val split =
        line
            .replace("bags", "")
            .replace("bag", "")
            .replace(".", "")
            .trim()
            .split("contain")
            .map { it.trim() }

    val mainBag = mutableListOf(Bag(split[0]))
    val offBags =
        split[1]
            .split(',')
            .map { it.trim() }
            .map {
                if (it.equals("no other")) {
                    null
                } else {
                    val name = it.dropWhile { c -> !c.isLetter() }
                    val count = it.split(" ")[0].toInt()

                    val b = Bag(name)
                    b.count = count
                    b
                }
            }
            .filterNotNull()

    mainBag.addAll(offBags)
    return mainBag
}

val bags = mutableMapOf<Bag, List<Bag>>()
val colors = mutableSetOf<String>()

fun countGold() {
    bags.entries.filter{ it.key.id != "shiny gold" }.forEach { countGoldRec(it.key.id, it.key, it.value) }
}

fun countGoldRec(first: String, bag: Bag, childs: List<Bag>) {
    if (bag.id == "shiny gold") {
        colors.add(first)
        return
    }

    childs.forEach { countGoldRec(first, it, bags.getOrDefault(bag, listOf())) }
}

lines.map(::getAllBagsOfLine).forEach {
    val main = it[0]
    val others = it.subList(1, it.size)

    bags.put(main, others)
}

countGold()
print(colors.size)