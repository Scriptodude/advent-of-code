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

fun count() {
    println(bags.getOrDefault(Bag("shiny gold"), listOf()).map { countBags(it) }.sum());
}

fun countBags(bag: Bag): Int {
    val actualChilds = bags.get(bag)
    if (actualChilds.isNullOrEmpty()) {
        return bag.count
    }

    return bag.count + actualChilds.map { bag.count * countBags(it) }.sum()
}

lines.map(::getAllBagsOfLine).forEach {
    val main = it[0]
    val others = it.subList(1, it.size)

    bags.put(main, others)
}

count()