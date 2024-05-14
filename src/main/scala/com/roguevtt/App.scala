package com.roguevtt

import zio._
import zio.http._

object App extends ZIOAppDefault{

  val routes: HttpApp[Any] =
    Routes(
      Method.GET / "" -> handler(Response.text("Greetings at your service")),
      Method.GET / "greet" -> handler { (req: Request) =>
        val name = req.queryParamToOrElse("name", "World")
        Response.text(s"Hello $name!")
      }
    ).toHttpApp

  def run: ZIO[Any, Throwable, Nothing] = Server.serve(routes).provide(Server.default)
}
