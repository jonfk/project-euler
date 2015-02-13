
import Stream._
import scala.math.BigInt

object three {
  def main(args : Array[String]) {
    val primes = sieve(from(2))
    // val p = primes.take(10000).toList
    // val p = birdPrimes.takeWhile( BigInt(_) < BigInt(600851475143L / 2)).toList

    // Too much overhead for storing numbers that have already been checked
    // def nums(n: Long) : Stream[Long]= n #:: nums(n - 1)
    // val result = nums(600851475143L - 1).find( x => 600851475143L % x == 0 && isPrime(birdPrimes, x))

  }

  def isPrime(primes : Stream[Int], num : Long) : Boolean = {
    primes.takeWhile(_ <= num)
    primes.last == num
  }

  // Naive sieve
  def sieve(s :Stream[Int]): Stream[Int] = s.head #:: sieve(s.tail filter (_ % s.head != 0))

  // From Rosetta Code
  // Odds-Only "infinite" generator sieve using Streams and Co-Inductive Streams
  // Read "The Genuine Sieve of Eratosthenes" by Melissa E. O'Neill
  def birdPrimes() = {
    def oddPrimes: Stream[Int] = {
      def merge(xs: Stream[Int], ys: Stream[Int]): Stream[Int] = {
        val (x, y) = (xs.head, ys.head)

        if (y > x)
          x #:: merge(xs.tail, ys)
        else if (x > y)
          y #:: merge(xs, ys.tail)
        else
          x #:: merge(xs.tail, ys.tail)
      }

      def primeMltpls(p: Int): Stream[Int] = Stream.iterate(p * p)(_ + p + p)

      def allMltpls(ps: Stream[Int]): Stream[Stream[Int]] = primeMltpls(ps.head) #:: allMltpls(ps.tail)

      def join(ams: Stream[Stream[Int]]): Stream[Int] = ams.head.head #:: merge(ams.head.tail, join(ams.tail))

      def oddPrms(n: Int, composites: Stream[Int]): Stream[Int] =
        if (n >= composites.head) oddPrms(n + 2, composites.tail) else n #:: oddPrms(n + 2, composites)

      //following uses a new recursive source of odd base primes
      3 #:: oddPrms(5, join(allMltpls(oddPrimes)))
    }
    2 #:: oddPrimes
  }
}
