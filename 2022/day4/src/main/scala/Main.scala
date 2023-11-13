import scala.io._

object Main {
  def main(args: Array[String]) = {
    val lines = Source.stdin.getLines()
    var ans = 0;
    while (lines.hasNext) {
      val line = lines.next()
      val pairs = line.split(',')
      val firstPair = pairs(0).split('-')
      val secondPair = pairs(1).split('-')
      val a = firstPair(0).toInt
      val b = firstPair(1).toInt
      val c = secondPair(0).toInt
      val d = secondPair(1).toInt
      if ((a <= c && b >= c) || (a <= d && b >= d) || (c <= a && d >= a) || (c <= b && d >= b)) {
        ans += 1
      }
    }
    println(ans)
  }
}
