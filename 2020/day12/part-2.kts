import java.io.File;
import kotlin.math.sin
import kotlin.math.cos
import kotlin.math.atan2

data class action(val what: Char, val amount: Int)
data class waypoint(var x: Long, var y: Long) {
    fun addAngle(angle: Int) {
        val currentAngle = atan2(-y.toDouble(), x.toDouble())
        val angleToRad = Math.toRadians(angle.toDouble())
        val radians = currentAngle + angleToRad
        val radius = Math.sqrt((this.x * this.x + this.y * this.y).toDouble())

        this.x = Math.round(radius * cos(radians))
        this.y = Math.round(radius * -sin(radians))
    }
}

val lines = File("input.txt").bufferedReader().readLines().map { action(it.first(), it.drop(1).toInt()) }

val waypoint = waypoint(10L, -1L)
var x = 0L
var y = 0L

lines.forEach {
    when(it.what) {
        'N' -> waypoint.y -= it.amount
        'S' -> waypoint.y += it.amount
        'E' -> waypoint.x += it.amount
        'W' -> waypoint.x -= it.amount
        'L' -> waypoint.addAngle(it.amount)
        'R' -> waypoint.addAngle(-it.amount)
        'F' -> {
            x += it.amount * waypoint.x
            y += it.amount * waypoint.y
        }
    }
}

println(Math.abs(x) + Math.abs(y))