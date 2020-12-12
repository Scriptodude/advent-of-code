import java.io.File;
import kotlin.math.sin
import kotlin.math.cos

data class action(val what: Char, val amount: Int)
val lines = File("input.txt").bufferedReader().readLines().map { action(it.first(), it.drop(1).toInt()) }

var lookingAt = 0
var x = 0L
var y = 0L

lines.forEach {
    when(it.what) {
        'N' -> y -= it.amount
        'S' -> y += it.amount
        'E' -> x += it.amount
        'W' -> x -= it.amount
        'L' -> lookingAt += it.amount
        'R' -> lookingAt -= it.amount
        'F' -> {
            println("$lookingAt for ${it.amount} => $x, $y")
            val radians = Math.toRadians(lookingAt.toDouble())
            y -= it.amount * Math.round(sin(radians))
            x += it.amount * Math.round(cos(radians))
            println("$x, $y")
        }
    }
}

println(Math.abs(x) + Math.abs(y))