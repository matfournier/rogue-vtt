import sbt.*

object Deps {

  object V {
    val zio = "2.0.22"
    val zioStreams = "2.0.22"
    val zioHttp = "3.0.0-RC6"
    val zioJson = "0.6.2"
    val neoType = "0.2.11"
  }

  object Libraries {
    val zio = "dev.zio" %% "zio" % V.zio
    val zioStreams = "dev.zio" %% "zio" % V.zioStreams
    val zioHttp = "dev.zio" %% "zio-http" % V.zioHttp
    val zioJson = "dev.zio" %% "zio-json" % "0.6.2"
    val neoType = "io.github.kitlangton" %% "neotype" % V.neoType

    val munit = "org.scalameta" %% "munit" % "0.7.29" % Test
    val munitZio = "com.github.poslegm" %% "munit-zio" % "0.0.2" % Test
  }
}
