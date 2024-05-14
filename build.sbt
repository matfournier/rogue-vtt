import Deps.*

ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.3.3"


lazy val root = (project in file("."))
  .settings(
    name := "roguevtt"
  )

libraryDependencies ++= Seq(
  Libraries.zio,
  Libraries.zioHttp,
  Libraries.zioJson,
  Libraries.zioStreams,
  Libraries.neoType,
  Libraries.munit,
  Libraries.munitZio
)