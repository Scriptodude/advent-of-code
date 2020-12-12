import java.io.File;

var lines = File("input.txt").bufferedReader().readLines();
val expected = listOf("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid")
var total = 0;

fun getNextPassport(): String {
    val ret = lines.takeWhile { l -> !l.isBlank() }
    lines = lines.dropWhile { l -> !l.isBlank() }
    lines = lines.drop(1);

    return ret.joinToString(" ")
}

while(true) {
    val passport = getNextPassport();
    if (passport.isBlank()) break;

    val fields = passport.split(" ").map { it.split(":") }
    if (expected.all { fields.any { f -> it == f[0] } } ) {
        total += 1;
    }
}

println(total)