import java.io.{BufferedReader, FileReader, StreamTokenizer}
import java.util

import scala.annotation.tailrec
import scala.collection.mutable.ArrayBuffer
import scala.util.Random
import scala.collection.JavaConversions._

object MainS {
  def main(args: Array[String]): Unit = {
    val N = 100000000
    val xs = new Array[Int](N)
    var random = N
    for (i <- 0 until N) {
      random ^= random << 13
      random ^= random >> 17
      random ^= random << 5
      xs(i) = random
    }
    for (i <- 0 until 100) {
      val start = System.currentTimeMillis()
      val res = average(xs)
      val end = System.currentTimeMillis()
      if (i % 10 == 0) {
        println(res + " " + (end - start))
      }
    }

  }
  def average(x: Array[Int]): Double = {
    x.reduce(_ + _) * 1.0 / x.size
  }
}

