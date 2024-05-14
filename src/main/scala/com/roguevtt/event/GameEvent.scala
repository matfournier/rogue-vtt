package com.roguevtt.event

import com.roguevtt.game.Entity
import zio.json._

case class Bounds(x: Int, xx: Int, y: Int, yy: Int) {
  def forEach(f: (Int, Int) => Unit): Unit =
    for {
      xxx <- Range.inclusive(x, xx)
      yyy <- Range.inclusive(y, yy)
    } yield f(xxx, yyy)

  def map[B](f: (Int, Int) => B): List[B] =
    (for {
      xxx <- Range.inclusive(x, xx)
      yyy <- Range.inclusive(y, yy)
    } yield f(xxx, yyy)).toList
}

object Bounds {
  implicit val decoder: JsonDecoder[Bounds] = DeriveJsonDecoder.gen[Bounds]
  implicit val encoder: JsonEncoder[Bounds] = DeriveJsonEncoder.gen[Bounds]
}

// I probably don't need gameId, that should be obvious. 
// level shouldn't be optional 
final case class GameMessage(event: GameEvent, gameId: String, user: Option[String], level: Option[String]) {

  // look into ZIO clock for timed messages
  def timed: TimedGameMessage = TimedGameMessage(this, System.currentTimeMillis())
}

final case class TimedGameMessage(msg: GameMessage, time: Long)


object GameMessage {
  implicit val decoder: JsonDecoder[GameMessage] = DeriveJsonDecoder.gen[GameMessage]
  implicit val encoder: JsonEncoder[GameMessage] = DeriveJsonEncoder.gen[GameMessage] 
}

sealed trait GameEvent
object GameEvent {
  final case class TilePlaced(x: Int, y: Int, tileset: Int, idx: Int) extends GameEvent
  final case class TileRemoved(x: Int, y: Int, tileset: Int, idx: Int) extends GameEvent
  final case class Fill(bounds: Bounds, tileset: Int, idx: Int) extends GameEvent
  final case class Clear(bounds: Bounds, tileset: Int, idx: Int) extends GameEvent
  final case class AddToken(entity: Entity) extends GameEvent
  final case class RemoveToken(entity: Entity) extends GameEvent
  final case class MoveToken(entity: Entity, to: (Int, Int)) extends GameEvent
  final case class TextMessage(user: String, msg: String) extends GameEvent

  implicit val decoder: JsonDecoder[GameEvent] = DeriveJsonDecoder.gen[GameEvent]
  implicit val encoder: JsonEncoder[GameEvent] = DeriveJsonEncoder.gen[GameEvent]
}

sealed trait ServerEvent

object ServerEvent {
  case object Persist extends ServerEvent
}