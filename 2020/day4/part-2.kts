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

        val isValid = fields.all {
            when(it[0]) {
                "byr" -> 1920 <= it[1].toInt() && it[1].toInt() <= 2002
                "iyr" -> 2010 <= it[1].toInt() && it[1].toInt() <= 2020
                "eyr" -> 2020 <= it[1].toInt() && it[1].toInt() <= 2030
                "hgt" -> it[1].length > 2 && {
                    val height = it[1].dropLast(2).toInt()
                    val unit = it[1].drop(it[1].length - 2);
                    when (unit) {
                        "cm" -> 150 <= height && height <= 193
                        "in" -> 59 <= height && height <= 76
                        else -> false
                    }
                } ()
                "hcl" -> it[1].matches("#[a-f0-9]{6}".toRegex())
                "ecl" -> listOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(it[1])
                "pid" -> it[1].matches("\\d{9}".toRegex())
                else -> true
            }
        }

        if (isValid) total += 1;
    }
}

println(total)