import scala.io._

object Main {
  def main(args: Array[String]) = {
    val scores = (('a' to 'z') ++ ('A' to 'Z')).zipWithIndex.toMap
    val lines = Source.stdin.getLines()
    var ans = 0
    while (lines.hasNext) {
      val line1 = lines.next()
      val line2 = lines.next()
      val line3 = lines.next()
      val common = line1 intersect line2 intersect line3
      ans += scores(common(0)) + 1
    }
    println(ans)
  }
}
